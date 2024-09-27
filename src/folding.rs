use ark_bn254::{Bn254, constraints::GVar, Fr, G1Projective as G1};
use ark_grumpkin::{constraints::GVar as GVar2, Projective as G2};
use rand::rngs::OsRng;
use sonobe::{
    commitment::{kzg::KZG, pedersen::Pedersen},
    folding::nova::{Nova, PreprocessorParam},
    FoldingScheme,
    frontend::circom::CircomFCircuit,
    transcript::poseidon::poseidon_canonical_config,
};

pub type NovaFolding =
    Nova<G1, GVar, G2, GVar2, CircomFCircuit<Fr>, KZG<'static, Bn254>, Pedersen<G2>, false>;

pub fn prepare_folding(
    circuit: &CircomFCircuit<Fr>,
    start_ivc_state: Vec<Fr>,
    rng: &mut OsRng,
) -> NovaFolding {
    let nova_preprocess_params =
        PreprocessorParam::new(poseidon_canonical_config::<Fr>(), circuit.clone());
    let nova_params = NovaFolding::preprocess(&mut *rng, &nova_preprocess_params)
        .expect("Failed to preprocess Nova");
    NovaFolding::init(&nova_params, circuit.clone(), start_ivc_state).expect("Failed to init Nova")
}
