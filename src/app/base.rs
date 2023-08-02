//! The main function which starts the app using the given [`HostConfiguration`].
use tokio;

use super::{tasks, AppState};

use crate::{
    config::{self, HostConfiguration},
    error::AppError,
};

use crate::logger;

/// Starts the host app.
///
/// Supply settings to this app using a [`HostConfiguration`]. Returns [`Ok(())`] if
/// gracefully terminated; [`Err<AppError>`] otherwise.
pub async fn run_app(config: HostConfiguration) -> Result<(), AppError> {
    let addr = config
        .addr
        .clone()
        .unwrap_or(config::DEFAULT_ADDR.to_owned());
    let port = config.port.unwrap_or(config::DEFAULT_PORT);
    let listen_target = format!("{addr}:{port}");

    #[cfg(feature="debug")]
    logger::warning("This is a DEBUG build: extra information will be printed and/or returned, including game states that are private!");

    #[cfg(feature = "simulate_failures")]
    if let Some(factor) = config.simulated_failure_factor.as_ref() {
        if factor > &0 {
            logger::warning(&format!("This is a SIMULATE FAILURES build: 1 in {factor} queries will end up being an arbitrary failure."));
        }
    }

    #[allow(unused_variables)]
    let app_state = AppState::new().configure(&config);
    let termination_token = app_state.token();

    let app = tasks::app_server::create_app(app_state).await; // This is just awaiting the creation of the app.
    logger::info(&format!("Ready to listen at {listen_target}...",));

    tokio::select! {
        result = app.listen(&listen_target) => {
            result.map_err(|err| AppError::ServerStartUpError { addr: listen_target, err })
        },
        result = termination_token.task() => {
            result
        }
    }
}
