use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use tide;

use crate::{error::AppError, logger, models::traits::IsAppHook};

use super::{AppState, PageVisit};

/// An [`AppHook`] that wraps a [`tide::Endpoint`], allowing something to be done to an
/// [`AppState`] before each [`tide::Endpoint::call()`].
pub struct AppHook<T> {
    locked_state: Arc<RwLock<AppState>>,
    end_point: T,
}

impl<T> AppHook<T>
where
    T: IsAppHook,
{
    /// Wraps an [`IsAppHook`]+[`tide::Endpoint`] to create a
    pub fn new(locked_state: Arc<RwLock<AppState>>) -> Self {
        let end_point = T::new(Arc::clone(&locked_state));

        Self {
            locked_state,
            end_point,
        }
    }
}

#[async_trait]
impl<T, State> tide::Endpoint<State> for AppHook<T>
where
    T: tide::Endpoint<State>,
    State: 'static + Clone + Send + Sync,
{
    async fn call(&self, req: tide::Request<State>) -> tide::Result {
        let client = req.host().map(str::to_owned);
        let path = req.url().path().to_string();
        let url = req.url().to_string();
        let res = self.end_point.call(req).await;

        match self.locked_state.write() {
            Ok(mut app_state) => app_state.log_visit(PageVisit::new(client, path, url, &res)),
            Err(_) => logger::error(&format!(
                "{}. Page Visit from {} to {} will not be logged.",
                AppError::LockPoisoned("AppState"),
                client.unwrap_or("<Unknown Client>".to_owned()),
                url,
            )),
        }

        res
    }
}
