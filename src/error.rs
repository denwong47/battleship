//! Module containing the main [error] type [`AppError`]
//!
//! [error]: `std::error::Error`
use serde_json;
use std::io;
use thiserror::Error;
use uuid::Uuid;

use crate::{logger, models::traits::AddDefaultHeaders};

#[allow(unused_imports)]
use crate::_doc;

/// A collection of all possible errors raised by the app.
#[derive(Debug, Error)]
pub enum AppError {
    /// A [lock] for an object is poisoned; execution cannot continue.
    ///
    /// This can occur when a thread [`panic`]s while holding a writable [guard] reference to the [lock].
    ///
    /// This error is irrecoverable; please restart the server.
    ///
    /// ### Status
    /// `500 INTERNAL SERVER ERROR`
    ///
    /// [lock]: `std::sync::RwLock`
    /// [guard]: `std::sync::RwLockWriteGuard`
    #[error("A lock for {0} is poisoned; execution cannot continue.")]
    LockPoisoned(&'static str),

    /// [`host.json`] was not found at the specified location.
    ///
    /// ### Status
    /// `500 INTERNAL SERVER ERROR`
    ///
    /// [`host.json`]: `_doc::hostjson`
    #[error("Configuration not found at {path}: {err}")]
    ConfigNotReadable { path: String, err: io::Error },

    /// [`host.json`] contains invalid JSON.
    ///
    /// ### Status
    /// `500 INTERNAL SERVER ERROR`
    ///
    /// [`host.json`]: `_doc::hostjson`
    #[error("Cannot parse Configuration file at {path}: {err}")]
    ConfigNotParsable {
        path: String,
        err: serde_json::Error,
    },

    /// The feature requested is only enabled in debug mode.
    ///
    /// ### Status
    /// `406 NOT ACCEPTABLE`
    ///
    /// [`host.json`]: `_doc::hostjson`
    #[error("This feature is only enabled for debug build.")]
    DebugOnlyFeature,

    /// The server failed to start. This is typically caused by the OS preventing the
    /// host from starting up due to port number clash etc.
    ///
    /// ### Status
    /// `500 INTERNAL SERVER ERROR`
    #[error("App at {addr} failed to start up: {err}")]
    ServerStartUpError { addr: String, err: io::Error },

    /// [`tide`] encountered an error. The error message from [`tide`] will be displayed.
    ///
    /// ### Status
    /// Varies. The [`tide::Error`] status code is used.
    #[error("HTTP host encountered an error: {0:?}")]
    TideError(tide::Error),

    /// A query parameter provided is invalid.
    ///
    /// ### Status
    /// `400 BAD REQUEST`
    #[error("Invalid parameters passed: {0:?}")]
    InvalidParameters(tide::Error),

    /// A mandatory query parameter is missing.
    ///
    /// ### Status
    /// `404 NOT FOUND`
    #[error("Missing parameters in query: {0:?}")]
    MissingParameters(Vec<&'static str>),

    /// The OS prevented the host from listening to `Ctrl-C` termination commands.
    ///
    /// ### Status
    /// `410 GONE`
    #[error("Cannot listen to Ctrl-C calls: {message}")]
    CtrlCError { message: String },

    /// A return value had been computed, but the object failed to serialize.
    ///
    /// ### Status
    /// `500 INTERNAL SERVER ERROR`
    #[error("JSON Serialization Failed.")]
    JSONSerializationError,

    /// The host is attempting to create a board with an existing [`Uuid`].
    ///
    /// This is an improbable cause of error; if you are unlucky enough to encounter
    /// this, simply retry the request.
    ///
    /// ### Status
    /// `500 INTERNAL SERVER ERROR`
    #[error("Board {uuid} duplicated.")]
    DuplicatedBoard { uuid: Uuid },

    /// The provided [`Uuid`] does not match any existing boards.
    ///
    /// ### Status
    /// `404 NOT FOUND`
    #[error("Board {uuid} is missing.")]
    MissingBoard { uuid: Uuid },

    /// The provided baord ID is not a well formatted [`Uuid`].
    ///
    /// ### Status
    /// `400 BAD REQUEST`
    #[error("{uuid_str} is not a valid Board UUID.")]
    InvalidBoard { uuid_str: String },

    /// The target board cannot be modified, as the game has already ended.
    ///
    /// ### Status
    /// `406 NOT ACCEPTABLE`
    #[error("Board {uuid} is frozen - the game has ended.")]
    FrozenBoard { uuid: Uuid },

    /// A termination request had been sent with an error message. The host will now
    /// terminate with the message as its error.
    ///
    /// ### Status
    /// `410 GONE`
    #[error("A remote host requested a termination with error: {message}")]
    RemoteRequestedTermination { message: String },

    /// A requested position is not available.
    ///
    /// ### Status
    /// `406 NOT ACCEPTABLE`
    #[error("Position ({x}, {y}) is already occupied.")]
    PositionOccupied { x: usize, y: usize },

    /// A requested position is out of the bounds of the board. Check the board size
    /// before retrying.
    ///
    /// ### Status
    /// `406 NOT ACCEPTABLE`
    #[error("Coordinates ({x}, {y}) is out of bounds.")]
    CoordinatesOutOfBounds { x: usize, y: usize },

    /// A strike report had been requested on an empty position that had not been struck
    /// before.
    ///
    /// *Internal error only; should be unreachable.*
    ///
    /// ### Status
    /// `500 INTERNAL SERVER ERROR`
    #[error("Nothing to report: strike had not occurred.")]
    StrikeReportRequestedOnEmpty,

    /// An unknown error had occurred. Refer to the context for more information.
    ///
    /// ### Status
    /// `500 INTERNAL SERVER ERROR`
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
        .add_default_headers()
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
