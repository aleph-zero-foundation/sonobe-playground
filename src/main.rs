use std::time::Instant;

use scenario_config::ScenarioConfig;

use crate::folding::{
    prepare_folding, verify_folding, FoldingSchemeExt, HyperNovaFolding, NovaFolding,
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
mod scenario_config;

fn scenario<FS: FoldingSchemeExt>(config: ScenarioConfig, rng: &mut impl rand::RngCore) {

    // ============== FOLDING PREPARATION ==========================================================

    let (mut folding, folding_vp) = measure("Prepare folding", || {
        prepare_folding::<FS>(&config.circuit, config.start_ivc_state.clone(), rng)
    });

    // ============== FOLDING ======================================================================

    for (i, external_inputs_at_step) in config.input().iter().enumerate() {
        measure(&format!("Prove_step {i}"), || {
            folding
                .prove_step(&mut *rng, external_inputs_at_step.clone(), None)
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
