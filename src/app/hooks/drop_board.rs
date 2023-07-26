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
pub struct DropBoardHook {
    locked_state: Arc<RwLock<AppState>>,
}

impl IsAppHook for DropBoardHook {
    /// Create a new [`DropBoardHook`] from a [`AppState`].
    fn new(locked_state: Arc<RwLock<AppState>>) -> Self {
        Self { locked_state }
    }
}

#[async_trait]
impl<State> Endpoint<State> for DropBoardHook
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
                            .write()
                            .map_err(|_| AppError::LockPoisoned("AppState"))
                            .and_then(|mut app_state| app_state.drop_board(uuid))
                    })
            })
            .build_response())
    }
}
