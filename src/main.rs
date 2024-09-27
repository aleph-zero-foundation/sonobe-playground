use std::time::Instant;

use ark_bn254::Fr;
use num_traits::identities::Zero;
use sonobe::FoldingScheme;

use crate::{circuit::create_circuit, folding::prepare_folding, input::prepare_input};

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
    let mut folding = measure("Prepare folding", || {
        prepare_folding(&circuit, start_ivc_state, &mut rng)
    });

    for (i, external_inputs_at_step) in prepare_input()[..5].iter().enumerate() {
        measure(&format!("Nova::prove_step {i}"), || {
            folding
                .prove_step(rng, external_inputs_at_step.clone(), None)
                .expect("Failed to prove step")
        });
    }
}
