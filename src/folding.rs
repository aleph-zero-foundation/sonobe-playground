use ark_bn254::{constraints::GVar, Bn254, Fr, G1Projective as G1};
use ark_grumpkin::{constraints::GVar as GVar2, Projective as G2};
use rand::rngs::OsRng;
use sonobe::{
    commitment::{kzg::KZG, pedersen::Pedersen},
    folding::nova::{Nova, PreprocessorParam},
    frontend::circom::CircomFCircuit,
    transcript::poseidon::poseidon_canonical_config,
    FoldingScheme,
};

pub type NovaFolding =
    Nova<G1, GVar, G2, GVar2, CircomFCircuit<Fr>, KZG<'static, Bn254>, Pedersen<G2>, false>;
pub type NovaVerifierParam =
    <NovaFolding as FoldingScheme<G1, G2, CircomFCircuit<Fr>>>::VerifierParam;

pub fn prepare_folding(
    circuit: &CircomFCircuit<Fr>,
    start_ivc_state: Vec<Fr>,
    rng: &mut OsRng,
) -> (NovaFolding, NovaVerifierParam) {
    let nova_preprocess_params =
        PreprocessorParam::new(poseidon_canonical_config::<Fr>(), circuit.clone());
    let nova_params = NovaFolding::preprocess(&mut *rng, &nova_preprocess_params)
        .expect("Failed to preprocess Nova");
    let folding = NovaFolding::init(&nova_params, circuit.clone(), start_ivc_state)
        .expect("Failed to init Nova");

    (folding, nova_params.1)
}

pub fn verify_folding(
    folding: &NovaFolding,
    folding_vp: NovaVerifierParam,
    start_ivc_state: Vec<Fr>,
    num_steps: u32,
) {
    let (running_instance, incoming_instance, cyclefold_instance) = folding.instances();
    NovaFolding::verify(
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
