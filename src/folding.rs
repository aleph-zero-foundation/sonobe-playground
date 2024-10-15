use ark_bn254::{constraints::GVar, Bn254, Fr, G1Projective as G1};
use ark_crypto_primitives::sponge::poseidon::PoseidonConfig;
use ark_grumpkin::{constraints::GVar as GVar2, Projective as G2};
use itertools::Itertools;
use sonobe::{
    commitment::{kzg::KZG, pedersen::Pedersen},
    folding::{hypernova::HyperNova, nova::Nova},
    frontend::circom::CircomFCircuit,
    transcript::poseidon::poseidon_canonical_config,
    FoldingScheme, MultiFolding,
};

pub type NovaFolding =
    Nova<G1, GVar, G2, GVar2, CircomFCircuit<Fr>, KZG<'static, Bn254>, Pedersen<G2>, false>;
pub type HyperNovaFolding<const M: usize, const N: usize> = HyperNova<
    G1,
    GVar,
    G2,
    GVar2,
    CircomFCircuit<Fr>,
    KZG<'static, Bn254>,
    Pedersen<G2>,
    M,
    N,
    false,
>;

pub struct StepInput<OtherInstances> {
    pub external_inputs: Vec<Fr>,
    pub other_instances: Option<OtherInstances>,
}

pub trait FoldingSchemeExt: FoldingScheme<G1, G2, CircomFCircuit<Fr>> {
    fn num_steps(num_inputs: usize) -> usize;

    fn prepreprocess(
        poseidon_config: PoseidonConfig<Fr>,
        circuit: CircomFCircuit<Fr>,
    ) -> Self::PreprocessorParam;

    fn transform_inputs(
        &self,
        full_input: Vec<Vec<Fr>>,
        initial_state: Vec<Fr>,
        rng: &mut impl rand::RngCore,
    ) -> Vec<StepInput<Self::MultiCommittedInstanceWithWitness>>;
}

impl FoldingSchemeExt for NovaFolding {
    fn num_steps(num_inputs: usize) -> usize {
        num_inputs // no multifolding
    }


    fn prepreprocess(
        poseidon_config: PoseidonConfig<Fr>,
        circuit: CircomFCircuit<Fr>,
    ) -> Self::PreprocessorParam {
        Self::PreprocessorParam::new(poseidon_config, circuit)
    }

    fn transform_inputs(
        &self,
        full_input: Vec<Vec<Fr>>,
        _initial_state: Vec<Fr>,
        _rng: &mut impl rand::RngCore,
    ) -> Vec<StepInput<Self::MultiCommittedInstanceWithWitness>> {
        full_input
            .into_iter()
            .map(|input| StepInput {
                external_inputs: input,
                other_instances: None,
            })
            .collect()
    }
}

impl<const M: usize, const N: usize> FoldingSchemeExt for HyperNovaFolding<M, N> {
    fn num_steps(num_inputs: usize) -> usize {
        let per_step = M + N - 1;
        assert_eq!(num_inputs % per_step, 0);
        num_inputs / per_step
    }

    fn prepreprocess(
        poseidon_config: PoseidonConfig<Fr>,
        circuit: CircomFCircuit<Fr>,
    ) -> Self::PreprocessorParam {
        Self::PreprocessorParam::new(poseidon_config, circuit)
    }

    fn transform_inputs(
        &self,
        full_input: Vec<Vec<Fr>>,
        initial_state: Vec<Fr>,
        rng: &mut impl rand::RngCore,
    ) -> Vec<StepInput<Self::MultiCommittedInstanceWithWitness>> {
        full_input
            .into_iter()
            .chunks(M + N - 1)
            .into_iter()
            .map(|chunk| {
                let chunk = chunk.collect::<Vec<_>>();
                let (running, rest) = chunk.split_at(M - 1);
                let (incoming, [single]) = rest.split_at(N - 1) else {
                    panic!("Invalid input chunk size");
                };

                let lcccs = running
                    .iter()
                    .map(|instance| {
                        self.new_running_instance(
                            &mut *rng,
                            initial_state.clone(),
                            instance.clone(),
                        )
                        .expect("Failed to create running instance")
                    })
                    .collect();

                let cccs = incoming
                    .iter()
                    .map(|instance| {
                        self.new_incoming_instance(
                            &mut *rng,
                            initial_state.clone(),
                            instance.clone(),
                        )
                        .expect("Failed to create incoming instance")
                    })
                    .collect();

                StepInput {
                    external_inputs: single.clone(),
                    other_instances: Some((lcccs, cccs)),
                }
            })
            .collect()
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
    num_inputs: usize,
) {
    let (running_instance, incoming_instance, cyclefold_instance) = folding.instances();
    FS::verify(
        folding_vp,
        start_ivc_state,
        folding.state(),
        Fr::from(FS::num_steps(num_inputs) as u32),
        running_instance,
        incoming_instance,
        cyclefold_instance,
    )
    .expect("Failed to verify folded proof");
}
