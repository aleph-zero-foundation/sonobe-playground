use ark_bn254::{constraints::GVar, Bn254, Fr, G1Projective as G1};
use ark_crypto_primitives::sponge::poseidon::PoseidonConfig;
use ark_grumpkin::{constraints::GVar as GVar2, Projective as G2};
use sonobe::{
    commitment::{kzg::KZG, pedersen::Pedersen},
    folding::{hypernova::HyperNova, nova::Nova},
    frontend::circom::CircomFCircuit,
    transcript::poseidon::poseidon_canonical_config,
    FoldingScheme,
};

pub type NovaFolding =
    Nova<G1, GVar, G2, GVar2, CircomFCircuit<Fr>, KZG<'static, Bn254>, Pedersen<G2>, false>;
pub type HyperNovaFolding = HyperNova<
    G1,
    GVar,
    G2,
    GVar2,
    CircomFCircuit<Fr>,
    KZG<'static, Bn254>,
    Pedersen<G2>,
    1,
    1,
    false,
>;

pub struct StepInput<OtherInstances> {
    pub external_inputs: Vec<Fr>,
    pub other_instances: Option<OtherInstances>,
}

pub trait FoldingSchemeExt: FoldingScheme<G1, G2, CircomFCircuit<Fr>> {
    fn prepreprocess(
        poseidon_config: PoseidonConfig<Fr>,
        circuit: CircomFCircuit<Fr>,
    ) -> Self::PreprocessorParam;

    fn transform_inputs(
        full_input: Vec<Vec<Fr>>,
    ) -> impl Iterator<Item = StepInput<Self::MultiCommittedInstanceWithWitness>>;
}

impl FoldingSchemeExt for NovaFolding {
    fn prepreprocess(
        poseidon_config: PoseidonConfig<Fr>,
        circuit: CircomFCircuit<Fr>,
    ) -> Self::PreprocessorParam {
        Self::PreprocessorParam::new(poseidon_config, circuit)
    }

    fn transform_inputs(
        full_input: Vec<Vec<Fr>>,
    ) -> impl Iterator<Item = StepInput<Self::MultiCommittedInstanceWithWitness>> {
        full_input.into_iter().map(|input| StepInput {
            external_inputs: input,
            other_instances: None,
        })
    }
}

impl FoldingSchemeExt for HyperNovaFolding {
    fn prepreprocess(
        poseidon_config: PoseidonConfig<Fr>,
        circuit: CircomFCircuit<Fr>,
    ) -> Self::PreprocessorParam {
        Self::PreprocessorParam::new(poseidon_config, circuit)
    }

    fn transform_inputs(
        full_input: Vec<Vec<Fr>>,
    ) -> impl Iterator<Item = StepInput<Self::MultiCommittedInstanceWithWitness>> {
        full_input.into_iter().map(|input| StepInput {
            external_inputs: input,
            other_instances: Some((vec![], vec![])),
        })
    }
}

pub fn prepare_folding<FS: FoldingSchemeExt>(
    circuit: &CircomFCircuit<Fr>,
    start_ivc_state: Vec<Fr>,
    rng: &mut impl rand::RngCore,
) -> (FS, FS::VerifierParam) {
    let preprocess_params = FS::prepreprocess(poseidon_canonical_config::<Fr>(), circuit.clone());
    let params =
        FS::preprocess(&mut *rng, &preprocess_params).expect("Failed to preprocess folding scheme");
    let folding =
        FS::init(&params, circuit.clone(), start_ivc_state).expect("Failed to init folding scheme");

    (folding, params.1)
}

pub fn verify_folding<FS: FoldingSchemeExt>(
    folding: &FS,
    folding_vp: FS::VerifierParam,
    start_ivc_state: Vec<Fr>,
    num_steps: u32,
) {
    let (running_instance, incoming_instance, cyclefold_instance) = folding.instances();
    FS::verify(
        folding_vp,
        start_ivc_state,
        folding.state(),
        Fr::from(num_steps),
        running_instance,
        incoming_instance,
        cyclefold_instance,
    )
    .expect("Failed to verify folded proof");
}
