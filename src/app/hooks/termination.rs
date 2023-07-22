use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use http_types::mime;
use serde::Deserialize;
use tide::{prelude::*, Endpoint};

use super::super::app_state::AppState;
use crate::{error::AppError, logger};

pub struct TerminationHook {
    locked_state: Arc<RwLock<AppState>>,
}

impl TerminationHook {
    /// Create a new [`TerminationHook`] from a [`TerminationToken`] behind an [`Arc`]
    /// reference.
    pub fn new(state: Arc<RwLock<AppState>>) -> Self {
        Self { locked_state: state }
    }
}

/// Possible queries for the [`TerminationHook`] [`Endpoint`].
#[derive(Clone, Debug, Deserialize)]
pub struct TerminationQuery {
    pub error: Option<String>,
}

#[async_trait]
impl<State> Endpoint<State> for TerminationHook
where
    State: Clone + Send + Sync + 'static,
{
    async fn call(&self, req: tide::Request<State>) -> tide::Result {
        // Try unpacking the query:
        let response = match req.query::<TerminationQuery>() {
            Ok(query) => {
                let response = tide::Response::builder(200)
                    .body(tide::Body::from_json(&json!(
                        {
                            "action": "termination",
                            "success": true,
                        }
                    ))?)
                    .content_type(mime::JSON)
                    .build();

                if let Some(error) = query.error {
                    // Failure termination.
                    // The remote host had requested this server to terminate. Typically
                    // these are used when a firmware update is rolled out etc.
                    match req.remote() {
                        Some(remote) => {
                            let message =
                                format!("Termination request from '{remote}', app error: {error}.");
                            logger::error(&message);
                        }
                        None => {
                            let message = format!(
                                "Termination request from unknown remote, app error: {error}."
                            );
                            logger::error(&message);
                        }
                    };

                    self.locked_state
                        .read()
                        .map_err(
                            |_| AppError::LockPoisoned("AppState")
                        )?
                        .token()
                        .notify(Err(AppError::CtrlCError { message: error }));
                } else {
                    self.locked_state
                    .read()
                    .map_err(
                        |_| AppError::LockPoisoned("AppState")
                    )?
                    .token().notify(Ok(()));
                }

                response
            }
            Err(err) => tide::Response::builder(400)
                .body(tide::Body::from_json(&json!(
                    {
                        "action": "termination",
                        "success": false,
                        "reason": &err.to_string(),
                    }
                ))?)
                .content_type(mime::JSON)
                .build(),
        };

        tide::Result::Ok(response)
    }
}
