use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tide::{self, Endpoint};
use uuid::Uuid;

use crate::{
    app::AppState,
    error::AppError,
    models::traits::{QueryParams, ResponseBuilder},
};

/// [`Endpoint`] for creating a new board.
#[allow(dead_code)]
pub struct BoardStatusHook {
    locked_state: Arc<RwLock<AppState>>,
}

/// Parameters for the above [`Endpoint`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BoardStatusParams {
    pub uuid: Option<Uuid>,
}
impl<State> QueryParams<State> for BoardStatusParams {}

impl BoardStatusHook {
    /// Create a new [`BoardStatusHook`] from a [`AppState`].
    pub fn new(locked_state: Arc<RwLock<AppState>>) -> Self {
        Self { locked_state }
    }
}

#[async_trait]
impl<State> Endpoint<State> for BoardStatusHook
where
    State: Clone + Send + Sync + 'static,
{
    async fn call(&self, req: tide::Request<State>) -> tide::Result {
        Ok(BoardStatusParams::parse_req(&req)
            .and_then(|params| {
                if let Some(uuid) = params.uuid {
                    self.locked_state
                        .read()
                        .map_err(|_| AppError::LockPoisoned("AppState"))
                        .and_then(|app_state| app_state.get_board_status(uuid))
                } else {
                    Err(AppError::MissingParameters(vec!["uuid"]))
                }
            })
            .build_response())
    }
}
