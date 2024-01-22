use color_eyre::eyre::Report;
use libheman::{
    find_by_code, find_by_substring,
    status_code_registry::{CODE_REGISTRY, UNOFFICIAL_CODE_REGISTRY},
};
use tracing::{info, instrument, trace};
use tracing_subscriber::{fmt, EnvFilter};

fn install_tracing() {
    fmt()
        .without_time()
        .with_line_number(true)
        .with_env_filter(
            EnvFilter::try_from_default_env()
                // .or_else(|_| EnvFilter::try_new("warning")) // upon release
                .or_else(|_| EnvFilter::try_new("trace"))
                .unwrap(),
        )
        .init();
}

#[instrument]
fn main() -> Result<(), Report> {
    install_tracing();
    color_eyre::install()?;
    trace!("Starting {}", env!("CARGO_PKG_NAME"));

    let item1 = find_by_code(100, &CODE_REGISTRY).unwrap();
    let item2 = find_by_substring("tea", &UNOFFICIAL_CODE_REGISTRY)
        .next()
        .unwrap();

    info!("Here be CLI");
    info!("Code {}", item1.1);
    info!("Substring {}", item2.1);
    trace!("Finishing {}", env!("CARGO_PKG_NAME"));
    Ok(())
}
