use scenario_config::ScenarioConfig;
use tracing::info_span;

use crate::{
    folding::{prepare_folding, verify_folding, FoldingSchemeExt, HyperNovaFolding, NovaFolding},
    logging::{init_logging},
};

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
    let (mut folding, folding_vp) = info_span!("Prepare folding")
        .in_scope(|| prepare_folding::<FS>(&config.circuit, start_state.clone(), rng));

    let input = info_span!("Transform input")
        .in_scope(|| folding.transform_inputs(config.input().to_vec(), start_state, &mut *rng));

    // ============== FOLDING ======================================================================

    for (i, step_input) in input.into_iter().enumerate() {
        info_span!("Folding step", step = i).in_scope(|| {
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

    info_span!("Folding verification").in_scope(|| {
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
