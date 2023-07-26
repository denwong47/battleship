use serde::{Deserialize, Serialize};
use tide;
use time::OffsetDateTime;

use crate::config;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageVisit {
    #[serde(with = "config::serde_offset_date_time")]
    pub timestamp: OffsetDateTime,
    pub client: Option<String>,
    pub path: String,
    pub url: String,
    pub status_code: Option<tide::StatusCode>,
    pub error: Option<String>,
}

impl PageVisit {
    /// Create a [`PageVisit`] instance from raw components.
    pub fn new(client: Option<String>, path: String, url: String, res: &tide::Result) -> Self {
        Self {
            timestamp: OffsetDateTime::now_utc(),
            client,
            path,
            url,
            status_code: { res.as_ref().map(|response| response.status()).ok() },
            error: { res.as_ref().map_err(|err| err.to_string()).err() },
        }
    }

    /// Create a [`PageVisit`] instance from a pair of [`tide::Request`] and
    /// [`tide::Response`]. The [`tide::Response`] must be wrapped in a [`tide::Result`]
    /// structure.
    pub fn from_request_and_response<State>(req: &tide::Request<State>, res: &tide::Result) -> Self
    where
        State: 'static + Clone + Send + Sync,
    {
        Self::new(
            req.host().map(str::to_owned),
            req.url().path().to_string(),
            req.url().to_string(),
            res,
        )
    }
}
