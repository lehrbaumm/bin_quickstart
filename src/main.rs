use std::sync::Once;

use color_eyre::Report;
use tracing::info;
use tracing_subscriber::EnvFilter;

static INIT: Once = Once::new();

fn main() -> Result<(), Report> {
    initial_setup()?;

    info!("Hello, world!");

    Ok(())
}

fn initial_setup() -> Result<(), Report> {
    let mut result: Result<(), Report> = Ok(());
    INIT.call_once(|| {
        if std::env::var("RUST_LIB_BACKTRACE").is_err() {
            std::env::set_var("RUST_LIB_BACKTRACE", "1")
        }
        if std::env::var("RUST_LOG").is_err() {
            #[cfg(not(debug_assertions))]
            std::env::set_var("RUST_LOG", "info");
            #[cfg(debug_assertions)]
            std::env::set_var("RUST_LOG", "debug");
        }
        result = color_eyre::install()
            .and_then(|_| {
                Ok(tracing_subscriber::fmt::fmt()
                    .with_env_filter(EnvFilter::from_default_env())
                    .init())
            });
    });
    result
}
