use scenario_config::ScenarioConfig;

use crate::{
    folding::{prepare_folding, verify_folding, FoldingSchemeExt, HyperNovaFolding, NovaFolding},
    logging::init_logging,
};
use crate::logging::measure;

mod circuit;
mod logging;

mod folding;
mod input;
mod scenario_config;

#[tracing::instrument(skip(config, rng))]
fn scenario<FS: FoldingSchemeExt>(
    config: ScenarioConfig,
    rng: &mut impl rand::RngCore,
    folding_scheme: &str,
) {
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
    init_logging();

    let mut rng = rand::rngs::OsRng;
    let config = ScenarioConfig::new();

    scenario::<NovaFolding>(config.clone(), &mut rng, "Nova");
    scenario::<HyperNovaFolding<1, 1>>(config.clone(), &mut rng, "HyperNova<1,1>");
    scenario::<HyperNovaFolding<2, 2>>(config.clone(), &mut rng, "HyperNova<2,2>");
}
