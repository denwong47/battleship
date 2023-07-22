use serde::Deserialize;
use tide;

use crate::error::AppError;

/// A trait for anything that can be used as a URL query.
pub trait QueryParams<State>
where
    Self: for<'de> Deserialize<'de> + Sized,
{
    /// Try parse a [`tide::Request`] instance.
    fn parse_req(req: &tide::Request<State>) -> Result<Self, AppError> {
        req.query::<Self>().map_err(AppError::InvalidParameters)
    }
}
