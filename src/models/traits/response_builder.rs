use http_types::mime;
use serde::Serialize;
use tide;

use crate::AppError;

pub trait ResponseBuilder {
    /// Build a [`tide::Response`] from this instance.
    fn build_response(&self) -> tide::Response;
}

impl<T> ResponseBuilder for Result<T, AppError>
where
    T: Serialize,
{
    /// Build a [`tide::Response`] from this [`Result`].
    fn build_response(&self) -> tide::Response {
        match self {
            Ok(item) => {
                let ser_result = serde_json::to_value(item)
                    .map_err(|_| AppError::JSONSerializationError)
                    .and_then(|value| tide::Body::from_json(&value).map_err(AppError::TideError));

                match ser_result {
                    Ok(body) => tide::Response::builder(tide::StatusCode::Ok)
                        .body(body)
                        .content_type(mime::JSON)
                        .build(),
                    Err(err) => err.into(),
                }
            }
            Err(err) => err.into(),
        }
    }
}
