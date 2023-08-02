//! A wrapper for [`tide::Endpoint`] which in itself is a [`tide::Endpoint`].
//!
//! Use this to perform standardised actions like logging and failure
//! simulations prior to or after any calls to all [`tide::Endpoint`]s.
//!
use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use tide;

use crate::{error::AppError, logger, models::traits::IsAppHook};

use super::{AppState, PageVisit};

#[cfg(feature = "simulate_failures")]
use super::hooks::{should_fail, SimulatedFailureHook};

/// An [`AppHook`] that wraps a [`tide::Endpoint`], allowing something to be done to an
/// [`AppState`] before each [`tide::Endpoint::call()`].
///
/// Example:
///
/// ```rust
/// use std::sync::{Arc, RwLock};
/// use async_trait::async_trait;
/// use crate::app::AppHook;
///
/// struct MyEndpoint {}
///
/// #[async_trait]
/// impl<State> tide::Endpoint<State> for MyEndpoint
/// where
///     State: 'static + Clone + Send + Sync,
/// {
///     async fn call(&self, _req: tide::Request<State>) -> tide::Result {
///         todo!()
///     }
/// }
///
/// impl IsAppHook for MyEndpoint {
///     fn new(_locked_state: Arc<RwLock<AppState>>) -> Self {
///         Self {}
///     }
/// }
///
/// /// Simulate creation of an `AppHook`.
/// fn endpoint_factory() -> AppHook<MyEndpoint> {
///     let locked_state = Arc::new(RwLock::new(AppState::default()));
///     AppHook::<MyEndpoint>::new(Arc::clone(&locked_state))
/// }
///
/// ```

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

        let simulated_failure_factor = if cfg!(feature = "simulate_failures") {
            self.locked_state
                .read()
                .map(|app_state| app_state.simulated_failure_factor)
                .unwrap_or_default()
        } else {
            None
        };
        let res = if simulated_failure_factor.map(should_fail).unwrap_or(false) {
            SimulatedFailureHook::new().call(req).await
        } else {
            self.end_point.call(req).await
        };

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
