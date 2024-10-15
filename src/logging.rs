use std::io;

use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};

fn get_filter() -> EnvFilter {
    EnvFilter::builder()
        .with_default_directive(LevelFilter::WARN.into())
        .from_env_lossy()
        .add_directive("playground=info".parse().unwrap())
}

pub fn init_logging() {
    tracing_subscriber::fmt()
        .with_writer(io::stdout)
        .with_target(false)
        .without_time()
        .with_env_filter(get_filter())
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .with_level(false)
        .json()
        .try_init()
        .expect("Failed to initialize logging");
}
