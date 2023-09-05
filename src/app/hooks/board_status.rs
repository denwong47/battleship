use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use tide::{self, Endpoint};
use uuid::Uuid;

use crate::{
    app::AppState,
    error::AppError,
    models::traits::{IsAppHook, ResponseBuilder},
};

/// [`Endpoint`] for creating a new board.
#[allow(dead_code)]
pub struct BoardStatusHook {
    locked_state: Arc<RwLock<AppState>>,
}

impl IsAppHook for BoardStatusHook {
    /// Create a new [`BoardStatusHook`] from a [`AppState`].
    fn new(locked_state: Arc<RwLock<AppState>>) -> Self {
        Self { locked_state }
    }
}

#[async_trait]
impl<State> Endpoint<State> for BoardStatusHook
where
    State: Clone + Send + Sync + 'static,
{
    async fn call(&self, req: tide::Request<State>) -> tide::Result {
        Ok(req
            .param("uuid")
            .map_err(|_| AppError::MissingParameters(vec!["uuid"]))
            .and_then(|uuid_str| {
                Uuid::parse_str(uuid_str)
                    .map_err(|_| AppError::InvalidBoard {
                        uuid_str: uuid_str.to_owned(),
                    })
                    .and_then(|uuid| {
                        self.locked_state
                            .read()
                            .map_err(|_| AppError::LockPoisoned("AppState"))
                            .and_then(|app_state| app_state.get_board_status(uuid))
                    })
            })
            .build_response())
    }
}