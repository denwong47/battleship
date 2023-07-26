use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use tide::{self, Endpoint};

use crate::{app::AppState, error::AppError, models::traits::ResponseBuilder};

/// [`Endpoint`] for creating a new board.
#[allow(dead_code)]
pub struct ListBoardsHook {
    locked_state: Arc<RwLock<AppState>>,
}

impl ListBoardsHook {
    /// Create a new [`ListBoardsHook`] from a [`AppState`].
    pub fn new(locked_state: Arc<RwLock<AppState>>) -> Self {
        Self { locked_state }
    }
}

#[async_trait]
impl<State> Endpoint<State> for ListBoardsHook
where
    State: Clone + Send + Sync + 'static,
{
    async fn call(&self, _req: tide::Request<State>) -> tide::Result {
        Ok(self
            .locked_state
            .read()
            .map_err(|_| AppError::LockPoisoned("AppState"))
            .and_then(|app_state| app_state.list_board_statuses())
            .build_response())
    }
}
