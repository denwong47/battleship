//! Module containing the main [error] type [`AppError`]
//!
//! [error]: `std::error::Error`
use http_types::mime;
use serde_json;
use std::io;
use thiserror::Error;
use uuid::Uuid;

use crate::logger;

/// A collection of all possible errors raised by the app.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("A lock for {0} is poisoned; execution cannot continue.")]
    LockPoisoned(&'static str),

    #[error("Configuration not found at {path}: {err}")]
    ConfigNotReadable { path: String, err: io::Error },

    #[error("Cannot parse Configuration file at {path}: {err}")]
    ConfigNotParsable {
        path: String,
        err: serde_json::Error,
    },

    #[error("This feature is only enabled for debug build.")]
    DebugOnlyFeature,

    #[error("App at {addr} failed to start up: {err}")]
    ServerStartUpError { addr: String, err: io::Error },

    #[error("HTTP host encountered an error: {0:?}")]
    TideError(tide::Error),

    #[error("Invalid parameters passed: {0:?}")]
    InvalidParameters(tide::Error),

    #[error("Missing parameters in query: {0:?}")]
    MissingParameters(Vec<&'static str>),

    #[error("Cannot listen to Ctrl-C calls: {message}")]
    CtrlCError { message: String },

    #[error("JSON Serialization Failed.")]
    JSONSerializationError,

    #[error("Board {uuid} duplicated.")]
    DuplicatedBoard { uuid: Uuid },

    #[error("Board {uuid} is missing.")]
    MissingBoard { uuid: Uuid },

    #[error("{uuid_str} is not a valid Board UUID.")]
    InvalidBoard { uuid_str: String },

    #[error("Board {uuid} is frozen - the game has ended.")]
    FrozenBoard { uuid: Uuid },

    #[error("A remote host requested a termination with error: {message}")]
    RemoteRequestedTermination { message: String },

    #[error("Position ({x}, {y}) is already occupied.")]
    PositionOccupied { x: usize, y: usize },

    #[error("Coordinates ({x}, {y}) is out of bounds.")]
    CoordinatesOutOfBounds { x: usize, y: usize },

    #[error("Nothing to report: strike had not occurred.")]
    StrikeReportRequestedOnEmpty,

    #[error("unknown error occurred: {context}")]
    Unknown { context: String },
}

macro_rules! expand_variants {
    (
        $((
            $variant:ident,
            $status:path
        )),*$(,)?
    ) => {

        impl AppError {
            /// Get the HTTP Status code for this error.
            ///
            /// Full list here: `<https://docs.rs/tide/latest/tide/enum.StatusCode.html>`
            pub fn error_code(&self) -> tide::StatusCode {
                match self.name().as_str() {
                    $(
                        stringify!($variant) => $status,
                    )*
                    "TideError" => if let Self::TideError(err) = self {
                        err.status()
                    } else {
                        unreachable!()
                    },
                    _ => tide::StatusCode::InternalServerError,
                }
            }

            /// Get the name of this error.
            pub fn name(&self) -> String {
                [" ", "("]
                .into_iter()
                .fold(
                    format!("{:?}", self),
                    |name, sep| {
                        name
                        .split_once(sep)
                        .map(|(prefix,_)| prefix.to_owned())
                        .unwrap_or(name)
                    }
                )
            }
        }

    };
}

expand_variants!(
    // (LockPoisoned, tide::StatusCode::InternalServerError),
    // (ConfigNotReadable, tide::StatusCode::InternalServerError),
    // (ConfigNotParsable, tide::StatusCode::InternalServerError),
    // (ServerStartUpError, tide::StatusCode::InternalServerError),
    (DebugOnlyFeature, tide::StatusCode::NotAcceptable),
    (InvalidParameters, tide::StatusCode::BadRequest),
    (MissingParameters, tide::StatusCode::NotFound),
    (CtrlCError, tide::StatusCode::Gone),
    // (JSONSerializationError, tide::StatusCode::InternalServerError),
    // (DuplicatedBoard, tide::StatusCode::InternalServerError),
    (FrozenBoard, tide::StatusCode::NotAcceptable),
    (InvalidBoard, tide::StatusCode::BadRequest),
    (MissingBoard, tide::StatusCode::NotFound),
    (RemoteRequestedTermination, tide::StatusCode::Gone),
    (PositionOccupied, tide::StatusCode::NotAcceptable),
    (CoordinatesOutOfBounds, tide::StatusCode::NotAcceptable),
    (
        StrikeReportRequestedOnEmpty,
        tide::StatusCode::InternalServerError
    ),
    (Unknown, tide::StatusCode::InternalServerError),
);

impl From<&AppError> for tide::Response {
    fn from(value: &AppError) -> Self {
        tide::Response::builder(
            value.error_code()
        )
        .body(tide::Body::from_json(
            &serde_json::json!({
                "error": value.name(),
                "message": value.to_string(),
            })
        ).unwrap_or_else(|_| {
            logger::error(
                &format!(
                    "{name} has occurred, but JSON serialization failed. The original error was {value:?}.",
                    name=value.name(),
                    value=value,
                )
            );

            tide::Body::from_json(
                &serde_json::json!({
                    "error": "JSONSerializationError",
                    "message": format!("{name} has occurred, but the error cannot be serialized. Check server logs.", name=value.name()),
                })
            )
            .unwrap()
        }))
        .content_type(mime::JSON)
        .build()
    }
}

impl From<AppError> for tide::Response {
    fn from(value: AppError) -> Self {
        (&value).into()
    }
}
#[cfg(test)]
mod app_error_to_response {
    use super::*;

    macro_rules! expand_tests {
        (
            $((
                $name:ident,
                $err:expr,
                $expected:expr
            )),*$(,)?
        ) => {
            $(
                #[test]
                fn $name() {
                    assert!(
                        tide::Response::from(&$err).status() == $expected
                    )
                }
            )*
        };
    }

    expand_tests!(
        (
            lock_poisoned,
            AppError::LockPoisoned(""),
            tide::StatusCode::InternalServerError
        ),
        (
            frozen_board,
            AppError::FrozenBoard {
                uuid: Uuid::new_v4()
            },
            tide::StatusCode::NotAcceptable
        ),
        (
            invalid_parameters,
            AppError::InvalidParameters(tide::Error::from_str(tide::StatusCode::Ok, "Ignored")),
            tide::StatusCode::BadRequest
        ),
        (
            tide_error,
            AppError::TideError(tide::Error::from_str(tide::StatusCode::ImATeapot, "")),
            tide::StatusCode::ImATeapot
        )
    );
}
