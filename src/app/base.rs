use tokio;

use super::{tasks, AppState};

use crate::{
    config::{self, HostConfiguration},
    error::AppError,
};

/// Starts the host app.
pub async fn run_app(config: HostConfiguration) -> Result<(), AppError> {
    let addr = config.addr.unwrap_or(config::DEFAULT_ADDR.to_owned());
    let port = config.port.unwrap_or(config::DEFAULT_PORT);
    let listen_target = format!("{addr}:{port}");

    #[allow(unused_variables)]
    let app_state = AppState::new();
    let termination_token = app_state.token();

    let app = tasks::app_server::create_app(app_state).await; // This is just awaiting the creation of the app.

    tokio::select! {
        result = app.listen(&listen_target) => {
            result.map_err(|err| AppError::ServerStartUpError { addr: listen_target, err })
        },
        result = termination_token.task() => {
            result
        }
    }
}
