use scenario_config::ScenarioConfig;
use tracing::info_span;

use crate::{
    folding::{prepare_folding, verify_folding, FoldingSchemeExt, HyperNovaFolding, NovaFolding},
    logging::init_logging,
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

    // ============== FOLDING ======================================================================

    for (i, multistep_input) in config.input().chunks(FS::MULTISTEP_SIZE).enumerate() {
        info_span!("Folding step", step = i).in_scope(|| {
            folding
                .prove_multistep(multistep_input.to_vec(), start_state.clone(), &mut *rng)
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
    scenario::<HyperNovaFolding<6, 1>>(config.clone(), &mut rng, "HyperNova<6,1>");
    scenario::<HyperNovaFolding<1, 6>>(config.clone(), &mut rng, "HyperNova<1,6>");
}
