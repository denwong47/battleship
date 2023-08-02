//! Special module only enabled if feature `simulate_failures` is enabled.
//!
//! This simulates network failures like Gateway Error or Request Timeouts, which would
//! normally not happen on a localhost.
use std::time::Duration;

use async_std::task;
use async_trait::async_trait;
use http_types::mime;
use rand::prelude::*;
use tide::{self, Endpoint};

#[allow(unused_imports)]
use crate::app::AppHook;
use crate::logger;

const REQUEST_TIMEOUT: u64 = 15;

/// Roll the dice to see if we should simulate a failure.
pub fn should_fail(factor: usize) -> bool {
    (factor > 0) && {
        let choice: usize = rand::thread_rng().gen_range(0..factor);

        choice == 0
    }
}

/// Special Hook which is a [`Endpoint`], but only for the consumption of [`AppHook`].
/// This simulates a network failure should [`AppHook`] choose to do so.
#[derive(Default)]
pub struct SimulatedFailureHook {}

impl SimulatedFailureHook {
    /// Instantiate a new [`SimulatedFailureHook`]. This is not necessary, but good for future field expansions.
    pub fn new() -> Self {
        Self::default()
    }

    /// Simulate a 502 Bad Gateway.
    async fn create_bad_gateway(&self) -> tide::Response {
        tide::Response::builder(tide::StatusCode::BadGateway)
            // Deliberately return an HTML instead of a JSON so JSON parsers will fail
            .body(tide::Body::from_string(
                "<h1>502 Bad Gateway</h1>".to_owned(),
            ))
            .content_type(mime::HTML)
            .build()
    }

    /// Simulate a 408 Request Timeout.
    async fn create_request_timeout(&self) -> tide::Response {
        logger::debug(&format!(
            "Simulating Request Timeout. Waiting for {REQUEST_TIMEOUT} seconds."
        ));

        // Default wait for 15 seconds. Nobody's gonna wait for that.
        task::sleep(Duration::from_secs(REQUEST_TIMEOUT)).await;

        tide::Response::builder(tide::StatusCode::RequestTimeout)
            // Deliberately return an HTML instead of a JSON so JSON parsers will fail
            .body(tide::Body::from_string(
                "<h1>408 Request Timeout</h1>".to_owned(),
            ))
            .content_type(mime::HTML)
            .build()
    }
}

#[async_trait]
impl<State> Endpoint<State> for SimulatedFailureHook
where
    State: Clone + Send + Sync + 'static,
{
    async fn call(&self, _req: tide::Request<State>) -> tide::Result {
        let choice: u8 = rand::thread_rng().gen_range(0..2);

        Ok(match choice {
            0 => self.create_bad_gateway().await,
            _ => self.create_request_timeout().await,
        })
    }
}
