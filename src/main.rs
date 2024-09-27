use std::time::Instant;

use ark_bn254::Fr;
use num_traits::identities::Zero;
use sonobe::FoldingScheme;
use crate::{
    circuit::create_circuit,
    folding::{prepare_folding, verify_folding},
    input::prepare_input,
};

fn measure<T, Action: FnOnce() -> T>(action_name: &str, action: Action) -> T {
    let start = Instant::now();
    let result = action();
    println!("{action_name}: {:?}", start.elapsed());
    result
}

mod circuit;
mod folding;
mod input;

fn main() {
    let mut rng = rand::rngs::OsRng;

    let circuit = measure("Prepare circuit", create_circuit);

    let start_ivc_state = vec![Fr::zero(); 2];
    let (mut folding, folding_vp) = measure("Prepare folding", || {
        prepare_folding(&circuit, start_ivc_state.clone(), &mut rng)
    });

    let num_steps = 5;
    for (i, external_inputs_at_step) in prepare_input()[..num_steps].iter().enumerate() {
        measure(&format!("Nova::prove_step {i}"), || {
            folding
                .prove_step(rng, external_inputs_at_step.clone(), None)
                .expect("Failed to prove step")
        });
    }

    measure("Folding verification", || {
        verify_folding(&folding, folding_vp, start_ivc_state, num_steps as u32)
    });
}
