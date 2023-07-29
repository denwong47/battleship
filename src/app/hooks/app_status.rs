use std::{
    ops::Deref,
    sync::{Arc, RwLock},
};

use async_trait::async_trait;
use tide::{self, Endpoint};

use crate::{
    app::AppState,
    error::AppError,
    models::traits::{IsAppHook, ResponseBuilder},
};

/// [`Endpoint`] for creating a new board.
#[allow(dead_code)]
pub struct AppStatusHook {
    locked_state: Arc<RwLock<AppState>>,
}

impl IsAppHook for AppStatusHook {
    /// Create a new [`AppStatusHook`] from a [`AppState`].
    fn new(locked_state: Arc<RwLock<AppState>>) -> Self {
        Self { locked_state }
    }
}

#[async_trait]
impl<State> Endpoint<State> for AppStatusHook
where
    State: Clone + Send + Sync + 'static,
{
    async fn call(&self, _req: tide::Request<State>) -> tide::Result {
        Ok(self
            .locked_state
            .read()
            .map_err(|_| AppError::LockPoisoned("AppState"))
            .and_then(|app_state| {
                serde_json::to_value(app_state.deref())
                    .map_err(|_| AppError::JSONSerializationError)
            })
            .build_response())
    }
}
