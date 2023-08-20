//! List all the strikes that occurred on a board.
use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use tide::{self, Endpoint};
use uuid::Uuid;

use crate::{
    app::AppState,
    error::AppError,
    models::traits::{IsAppHook, ResponseBuilder},
};

#[allow(unused_imports)]
use crate::models::{Board, ShipIntel, Strike};

/// [`Endpoint`] for listing all the [`Strike`]s that occurred on a board.
///
/// This is not an accurate "replay" of all the [`Strike`]s. It differs by two ways:
///
/// - the [`ShipIntel`] will always reflect the current state of the ship, not when the
///   [`Strike`] occurred.
/// - the `ships_remaining` field is not populated; it will always be `null`.
///
/// A typical use of this [`Endpoint`] is for a client to render the state of a
/// [`Board`] without any prior knowledge of what happened. In such a scenario, tracing
/// the history and progress of the board accurately is not required.
#[allow(dead_code)]
pub struct ListStrikesHook {
    locked_state: Arc<RwLock<AppState>>,
}

impl IsAppHook for ListStrikesHook {
    /// Create a new [`ListStrikesHook`] from a [`AppState`].
    fn new(locked_state: Arc<RwLock<AppState>>) -> Self {
        Self { locked_state }
    }
}

#[async_trait]
impl<State> Endpoint<State> for ListStrikesHook
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
                            .and_then(|app_state| app_state.get_board_strikes(uuid))
                    })
            })
            .build_response())
    }
}
