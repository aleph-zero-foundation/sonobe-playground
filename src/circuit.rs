use std::env::current_dir;
use ark_bn254::Fr;
use sonobe::frontend::circom::CircomFCircuit;
use sonobe::frontend::FCircuit;

const IVC_STEP_WIDTH: usize = 2;
const STEP_INPUT_WIDTH: usize = 256;

pub fn create_circuit() -> CircomFCircuit<Fr> {
    let root = current_dir().expect("Failed to get current directory");
    let circuit_file = root.join("circuit/grayscale_step.r1cs");
    let witness_generator_file = root.join("circuit/grayscale_step_js/grayscale_step.wasm");

    let f_circuit_params = (
        circuit_file.into(),
        witness_generator_file.into(),
        IVC_STEP_WIDTH,
        STEP_INPUT_WIDTH,
    );
    CircomFCircuit::<Fr>::new(f_circuit_params).expect("Failed to create circuit")
}
