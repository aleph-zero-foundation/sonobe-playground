use std::env::current_dir;

use ark_bn254::Fr;
use experimental_frontends::circom::CircomFCircuit;
use sonobe::frontend::FCircuit;

const IVC_STEP_WIDTH: usize = 2;
pub(crate) const STEP_INPUT_WIDTH: usize = 256;

pub fn create_circuit() -> CircomFCircuit<Fr, STEP_INPUT_WIDTH> {
    let root = current_dir().expect("Failed to get current directory");
    let circuit_file = root.join("circuit/grayscale_step.r1cs");
    let witness_generator_file = root.join("circuit/grayscale_step_js/grayscale_step.wasm");

    let f_circuit_params = (
        circuit_file.into(),
        witness_generator_file.into(),
        IVC_STEP_WIDTH,
    );
    CircomFCircuit::<Fr, STEP_INPUT_WIDTH>::new(f_circuit_params).expect("Failed to create circuit")
}
