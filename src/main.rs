use std::time::Instant;

use scenario_config::ScenarioConfig;
use sonobe::FoldingScheme;

use crate::folding::{prepare_folding, verify_folding};

fn measure<T, Action: FnOnce() -> T>(action_name: &str, action: Action) -> T {
    let start = Instant::now();
    let result = action();
    println!("{action_name}: {:?}", start.elapsed());
    result
}

mod circuit;
mod folding;
mod input;
mod scenario_config;

fn main() {
    let mut rng = rand::rngs::OsRng;
    let config = ScenarioConfig::new();

    let (mut folding, folding_vp) = measure("Prepare folding", || {
        prepare_folding(&config.circuit, config.start_ivc_state.clone(), &mut rng)
    });

    for (i, external_inputs_at_step) in config.input().iter().enumerate() {
        measure(&format!("Nova::prove_step {i}"), || {
            folding
                .prove_step(rng, external_inputs_at_step.clone(), None)
                .expect("Failed to prove step")
        });
    }

    measure("Folding verification", || {
        verify_folding(
            &folding,
            folding_vp,
            config.start_ivc_state,
            config.num_steps as u32,
        )
    });
}
