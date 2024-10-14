use std::time::Instant;

use scenario_config::ScenarioConfig;

use crate::folding::{
    prepare_folding, verify_folding, FoldingSchemeExt, HyperNovaFolding, NovaFolding,
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

    let start_state = config.start_ivc_state.clone();
    let (mut folding, folding_vp) = measure("Prepare folding", || {
        prepare_folding::<FS>(&config.circuit, start_state.clone(), rng)
    });

    // ============== FOLDING ======================================================================

    let input = measure("Transform input", || {
        folding.transform_inputs(config.input().to_vec(), start_state.clone(), &mut *rng)
    });
    for (i, step_input) in input.into_iter().enumerate() {
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
            config.num_inputs,
        )
    });
}

fn main() {
    let mut rng = rand::rngs::OsRng;
    let config = ScenarioConfig::new();

    println!("========== Nova folding scheme ====================");
    scenario::<NovaFolding>(config.clone(), &mut rng);

    println!("========== HyperNova<1,1> folding scheme ==========");
    scenario::<HyperNovaFolding<1, 1>>(config.clone(), &mut rng);

    println!("========== HyperNova<2,2> folding scheme ==========");
    scenario::<HyperNovaFolding<2, 2>>(config, &mut rng);
}
