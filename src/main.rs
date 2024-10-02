use std::time::Instant;

use scenario_config::ScenarioConfig;

use crate::folding::{
    FoldingSchemeExt, HyperNovaFolding, NovaFolding, prepare_folding, verify_folding,
};

mod circuit;

mod folding;
mod input;
mod scenario_config;

fn measure<T, Action: FnOnce() -> T>(action_name: &str, action: Action) -> T {
    let start = Instant::now();
    let result = action();
    println!("{action_name}: {:?}", start.elapsed());
    result
}

fn scenario<FS: FoldingSchemeExt>(config: ScenarioConfig, rng: &mut impl rand::RngCore) {
    // ============== FOLDING PREPARATION ==========================================================

    let (mut folding, folding_vp) = measure("Prepare folding", || {
        prepare_folding::<FS>(&config.circuit, config.start_ivc_state.clone(), rng)
    });

    // ============== FOLDING ======================================================================

    let input = config.input().to_vec();
    for (i, step_input) in FS::transform_inputs(input).enumerate() {
        measure(&format!("Prove_step {i}"), || {
            folding
                .prove_step(
                    &mut *rng,
                    step_input.external_inputs,
                    step_input.other_instances,
                )
                .expect("Failed to prove step")
        });
    }

    // ============== FOLDING VERIFICATION =========================================================

    measure("Folding verification", || {
        verify_folding(
            &folding,
            folding_vp,
            config.start_ivc_state,
            config.num_steps as u32,
        )
    });
}

fn main() {
    let mut rng = rand::rngs::OsRng;
    let config = ScenarioConfig::new();

    println!("========== Nova folding scheme ==========");
    scenario::<NovaFolding>(config.clone(), &mut rng);

    println!("========== HyperNova folding scheme ==========");
    scenario::<HyperNovaFolding>(config, &mut rng);
}
