pub mod app;
pub mod args;
pub mod config;
pub mod error;
pub mod logger;
pub mod models;

use crate::error::AppError;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Resolve configurations
    let cli_args = args::CommandLineParameters::parse();
    let config = config::HostConfiguration::try_from_args(&cli_args)
        .unwrap_or_default()
        .resolve_with_args(&cli_args);

    logger::info("Starting `battleship` server.");

    let result = app::run_app(config).await;

    match &result {
        Ok(()) => logger::info("Terminating `battleship` server gracefully."),
        Err(err) => logger::error(&format!(
            "Terminating `battleship` server with error: {err}"
        )),
    }

    result
}
