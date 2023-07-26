use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tide::{self, Endpoint};
use uuid::Uuid;

use crate::{
    app::AppState,
    error::AppError,
    models::{
        traits::{QueryParams, ResponseBuilder},
        Coordinates, Strike,
    },
};

/// [`Endpoint`] for creating a new board.
#[allow(dead_code)]
pub struct StrikeHook {
    locked_state: Arc<RwLock<AppState>>,
}

/// Parameters for the above [`Endpoint`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StrikeParams {
    pub uuid: Option<Uuid>,
    pub x: Option<usize>,
    pub y: Option<usize>,
}
impl<State> QueryParams<State> for StrikeParams {}

impl StrikeHook {
    /// Create a new [`StrikeHook`] from a [`AppState`].
    pub fn new(locked_state: Arc<RwLock<AppState>>) -> Self {
        Self { locked_state }
    }
}

#[async_trait]
impl<State> Endpoint<State> for StrikeHook
where
    State: Clone + Send + Sync + 'static,
{
    async fn call(&self, req: tide::Request<State>) -> tide::Result {
        Ok(StrikeParams::parse_req(&req)
            .and_then(|params| match (params.uuid, params.x, params.y) {
                (Some(uuid), Some(x), Some(y)) => self
                    .locked_state
                    .read()
                    .map_err(|_| AppError::LockPoisoned("AppState"))
                    .and_then(|app_state| {
                        app_state
                            .get_board(uuid)?
                            .write()
                            .map_err(|_| AppError::LockPoisoned("Board"))?
                            .add_strike(Strike::new(Coordinates::new(x, y)))
                    }),
                _ => Err(AppError::MissingParameters(vec!["uuid", "x", "y"])),
            })
            .build_response())
    }
}
