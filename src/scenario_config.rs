use ark_bn254::Fr;
use num_traits::Zero;
use sonobe::frontend::circom::CircomFCircuit;
use tracing::info_span;

use crate::{circuit::create_circuit, input::prepare_input};

#[derive(Clone)]
pub struct ScenarioConfig {
    pub num_inputs: usize,
    pub start_ivc_state: Vec<Fr>,
    pub circuit: CircomFCircuit<Fr>,
    input: Vec<Vec<Fr>>,
}

impl ScenarioConfig {
    pub fn new() -> Self {
        Self {
            num_inputs: 6,
            start_ivc_state: vec![Fr::zero(); 2],
            circuit: info_span!("Prepare circuit").in_scope(create_circuit),
            input: info_span!("Prepare input").in_scope(prepare_input),
        }
    }

    pub fn input(&self) -> &[Vec<Fr>] {
        &self.input[..self.num_inputs]
    }
}
