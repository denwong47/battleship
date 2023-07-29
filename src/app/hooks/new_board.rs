use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tide::{self, Endpoint};

use crate::{
    app::AppState,
    config,
    error::AppError,
    models::traits::{IsAppHook, QueryParams, ResponseBuilder},
};

/// [`Endpoint`] for creating a new board.
#[allow(dead_code)]
pub struct NewBoardHook {
    locked_state: Arc<RwLock<AppState>>,
}

/// Parameters for the above [`Endpoint`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NewBoardParams {
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub ship_count: Option<u16>,
}
impl<State> QueryParams<State> for NewBoardParams {}

impl IsAppHook for NewBoardHook {
    /// Create a new [`NewBoardHook`] from a [`AppState`] behind a [`RwLock`].
    fn new(locked_state: Arc<RwLock<AppState>>) -> Self {
        Self { locked_state }
    }
}

#[async_trait]
impl<State> Endpoint<State> for NewBoardHook
where
    State: Clone + Send + Sync + 'static,
{
    async fn call(&self, req: tide::Request<State>) -> tide::Result {
        Ok(NewBoardParams::parse_req(&req)
            .and_then(|params| {
                let size = [
                    params.width.unwrap_or(config::DEFAULT_BOARD_SIZE[0]),
                    params.height.unwrap_or(config::DEFAULT_BOARD_SIZE[1]),
                ];
                let ship_count = params.ship_count.unwrap_or(config::DEFAULT_SHIP_COUNT);

                self.locked_state
                    .write()
                    .map_err(|_| AppError::LockPoisoned("AppState"))
                    .and_then(|mut app_state| app_state.new_board(size, ship_count))
                    .map(|uuid| {
                        serde_json::json!({
                            "action": "createGame",
                            "success": true,
                            "game": uuid,
                        })
                    })
            })
            .build_response())
    }
}
