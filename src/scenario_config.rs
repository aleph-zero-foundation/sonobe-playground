use ark_bn254::Fr;
use experimental_frontends::circom::CircomFCircuit;
use num_traits::Zero;
use tracing::info_span;

use crate::{
    circuit::{create_circuit, STEP_INPUT_WIDTH},
    input::prepare_input,
};

#[derive(Clone)]
pub struct ScenarioConfig {
    pub num_inputs: usize,
    pub start_ivc_state: Vec<Fr>,
    pub circuit: CircomFCircuit<Fr, STEP_INPUT_WIDTH>,
    input: Vec<Vec<Fr>>,
}

impl ScenarioConfig {
    pub fn new() -> Self {
        Self {
            num_inputs: 360,
            start_ivc_state: vec![Fr::zero(); 2],
            circuit: info_span!("Prepare circuit").in_scope(create_circuit),
            input: info_span!("Prepare input").in_scope(prepare_input),
        }
    }

    pub fn input(&self) -> &[Vec<Fr>] {
        &self.input[..self.num_inputs]
    }
}
