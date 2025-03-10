use ark_bn254::{Bn254, Fr, G1Projective as G1};
use ark_crypto_primitives::sponge::poseidon::PoseidonConfig;
use ark_grumpkin::Projective as G2;
use experimental_frontends::{circom::CircomFCircuit, utils::VecF};
use sonobe::{
    commitment::{kzg::KZG, pedersen::Pedersen},
    folding::{hypernova::HyperNova, nova::Nova},
    transcript::poseidon::poseidon_canonical_config,
    Error, FoldingScheme, MultiFolding,
};
use tracing::info_span;

use crate::circuit::STEP_INPUT_WIDTH;

pub type NovaFolding =
    Nova<G1, G2, CircomFCircuit<Fr, STEP_INPUT_WIDTH>, KZG<'static, Bn254>, Pedersen<G2>, false>;
pub type HyperNovaFolding<const M: usize, const N: usize> = HyperNova<
    G1,
    G2,
    CircomFCircuit<Fr, STEP_INPUT_WIDTH>,
    KZG<'static, Bn254>,
    Pedersen<G2>,
    M,
    N,
    false,
>;

pub struct StepInput<OtherInstances> {
    pub external_inputs: VecF<Fr, STEP_INPUT_WIDTH>,
    pub other_instances: Option<OtherInstances>,
}

pub trait FoldingSchemeExt: FoldingScheme<G1, G2, CircomFCircuit<Fr, STEP_INPUT_WIDTH>> {
    const MULTISTEP_SIZE: usize;

    fn num_steps(num_inputs: usize) -> usize {
        assert_eq!(num_inputs % Self::MULTISTEP_SIZE, 0);
        num_inputs / Self::MULTISTEP_SIZE
    }

    fn prepreprocess(
        poseidon_config: PoseidonConfig<Fr>,
        circuit: CircomFCircuit<Fr, STEP_INPUT_WIDTH>,
    ) -> Self::PreprocessorParam;

    fn transform_multi_input(
        &self,
        multi_input: Vec<Vec<Fr>>,
        initial_state: Vec<Fr>,
        rng: &mut impl rand::RngCore,
    ) -> StepInput<Self::MultiCommittedInstanceWithWitness>;

    fn prove_multistep(
        &mut self,
        multi_input: Vec<Vec<Fr>>,
        initial_state: Vec<Fr>,
        rng: &mut impl rand::RngCore,
    ) -> Result<(), Error> {
        let step_input = info_span!("Input prep")
            .in_scope(|| self.transform_multi_input(multi_input, initial_state, rng));

        info_span!("Proving").in_scope(|| {
            self.prove_step(rng, step_input.external_inputs, step_input.other_instances)
        })
    }
}

impl FoldingSchemeExt for NovaFolding {
    const MULTISTEP_SIZE: usize = 1;

    fn prepreprocess(
        poseidon_config: PoseidonConfig<Fr>,
        circuit: CircomFCircuit<Fr, STEP_INPUT_WIDTH>,
    ) -> Self::PreprocessorParam {
        Self::PreprocessorParam::new(poseidon_config, circuit)
    }

    fn transform_multi_input(
        &self,
        input: Vec<Vec<Fr>>,
        _initial_state: Vec<Fr>,
        _rng: &mut impl rand::RngCore,
    ) -> StepInput<Self::MultiCommittedInstanceWithWitness> {
        assert_eq!(input.len(), 1);
        StepInput {
            external_inputs: VecF(input[0].clone()),
            other_instances: None,
        }
    }
}

impl<const M: usize, const N: usize> FoldingSchemeExt for HyperNovaFolding<M, N> {
    const MULTISTEP_SIZE: usize = M + N - 1;

    fn prepreprocess(
        poseidon_config: PoseidonConfig<Fr>,
        circuit: CircomFCircuit<Fr, STEP_INPUT_WIDTH>,
    ) -> Self::PreprocessorParam {
        Self::PreprocessorParam::new(poseidon_config, circuit)
    }

    fn transform_multi_input(
        &self,
        multi_input: Vec<Vec<Fr>>,
        initial_state: Vec<Fr>,
        rng: &mut impl rand::RngCore,
    ) -> StepInput<Self::MultiCommittedInstanceWithWitness> {
        let (running, rest) = multi_input.split_at(M - 1);
        let (incoming, [single]) = rest.split_at(N - 1) else {
            panic!("Invalid input chunk size");
        };

        let new_running = |instance| {
            self.new_running_instance(&mut *rng, initial_state.clone(), VecF(instance))
                .expect("Failed to create running instance")
        };

        let new_instances =
            |instances: Vec<Vec<Fr>>, maker| instances.into_iter().map(maker).collect();

        let lcccs = new_instances(running.to_vec(), new_running);

        let cccs = incoming
            .iter()
            .map(|instance| {
                self.new_incoming_instance(&mut *rng, initial_state.clone(), VecF(instance.clone()))
                    .expect("Failed to create incoming instance")
            })
            .collect();

        StepInput {
            external_inputs: VecF(single.clone()),
            other_instances: Some((lcccs, cccs)),
        }
    }
}

pub fn prepare_folding<FS: FoldingSchemeExt>(
    circuit: &CircomFCircuit<Fr, STEP_INPUT_WIDTH>,
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

pub fn verify_folding<FS: FoldingSchemeExt>(folding: &FS, folding_vp: FS::VerifierParam) {
    let ivc_proof = folding.ivc_proof();
    FS::verify(folding_vp, ivc_proof).expect("Failed to verify folded proof");
}
