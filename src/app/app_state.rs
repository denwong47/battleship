//! Stores the state of the app.
//!
//! The `battleship` is a tiny app that is designed entirely to run in memory.
//! This prevents most if not all ways of cheating unless a way is devised to
//! read into the app's allocated memory. This being written in Rust minimised
//! that risk.
//!
//! However this goes against the principle of REST in that calls to the endpoints of
//! this app is somewhat stateful, due to manipulation of board state within the
//! app, as opposed to an external database, for instance.
//!
//! This presents unique challenges to the architecture of this app. Many Rust
//! frameworks such as Actix requires the `Handler` to be `'static`, which would make it
//! impossible for this app.
//!
//! Luckily [`tide`] imposes no lifetime restrictions on [`tide::Endpoint`], thus
//! allowing any structs that implement the trait to have instances that are valid
//! [`tide::Endpoint`]s. By passing around a [`AppState`] to all these
//! [`tide::Endpoint`]s, an app state can be shared and maintained.
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, RwLock,
    },
};

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    config,
    error::AppError,
    logger,
    models::{Board, BoardStatus, StrikeReport},
};

use super::{tasks::termination::TerminationToken, PageVisit};

#[allow(unused_imports)]
use crate::models::traits::IsAppHook;

/// Records the current state of the app, to be passed during instantiation of
/// [`IsAppHook`].
///
/// Typically used behind a [`Arc<RwLock<AppState>>`], so that it can be safely
/// [`Send`] and [`Sync`].
#[derive(Debug, Serialize, Deserialize)]
pub struct AppState {
    #[serde(with = "config::serde_offset_date_time")]
    start_time: OffsetDateTime,
    #[serde(skip)]
    termination_token: Arc<TerminationToken>,
    #[serde(skip)]
    boards: HashMap<Uuid, RwLock<Board>>,
    page_visits: HashMap<String, AtomicU64>,
    #[cfg(feature = "debug")]
    visit_log: Vec<PageVisit>,
    #[cfg(feature = "simulate_failures")]
    pub simulated_failure_factor: Option<usize>,
}

impl Default for AppState {
    /// Create a default [`AppState`] using the current UTC time.
    fn default() -> Self {
        Self {
            start_time: OffsetDateTime::now_utc(),
            termination_token: Arc::new(TerminationToken::default()),
            boards: HashMap::new(),
            page_visits: HashMap::new(),
            #[cfg(feature = "debug")]
            visit_log: Vec::new(),
            #[cfg(feature = "simulate_failures")]
            simulated_failure_factor: None,
        }
    }
}

impl AppState {
    /// Create a default [`AppState`] using the current UTC time.
    pub fn new() -> Self {
        Self::default()
    }

    /// Configure this [`AppState`] with the provided [`config::HostConfiguration`].
    pub fn configure(mut self, host_config: &config::HostConfiguration) -> Self {
        #[cfg(feature = "simulate_failures")]
        {
            self.simulated_failure_factor = host_config.simulated_failure_factor;
        }

        self
    }

    /// The [`time::Duration`] that had elapsed since the start of the app.
    pub fn elapsed(&self) -> time::Duration {
        OffsetDateTime::now_utc() - self.start_time
    }

    /// Get a new [`Arc`] reference to the internal termination token.
    pub fn token(&self) -> Arc<TerminationToken> {
        Arc::clone(&self.termination_token)
    }

    /// Add a new [`Board`] and insert it into the this app.
    pub fn add_board(&mut self, board: Board) -> Result<Uuid, AppError> {
        let uuid = board.uuid();

        if self.boards.insert(uuid, RwLock::new(board)).is_some() {
            Err(AppError::DuplicatedBoard { uuid })
        } else {
            Ok(uuid)
        }
    }

    /// Create a new [`Board`] and insert it into this app.
    pub fn new_board(
        &mut self,
        size: [usize; 2],
        ship_count: u16,
    ) -> Result<BoardStatus, AppError> {
        let result = self.add_board(Board::new(size, ship_count));

        result.and_then(|uuid| {
            self.boards
                .get(&uuid)
                .ok_or(AppError::MissingBoard { uuid })
                .and_then(|board| {
                    board
                        .read()
                        .map_err(|_| AppError::LockPoisoned("Board"))?
                        .status()
                })
        })
    }

    /// Drop a board by [`Uuid`].
    pub fn drop_board(&mut self, uuid: Uuid) -> Result<Board, AppError> {
        self.boards
            .remove(&uuid)
            .ok_or(AppError::MissingBoard { uuid })
            .and_then(|lock| {
                lock.into_inner()
                    .map_err(|_| AppError::LockPoisoned("Board"))
            })
    }

    /// Get a board by [`Uuid`].
    pub fn get_board(&self, uuid: Uuid) -> Result<&RwLock<Board>, AppError> {
        self.boards
            .get(&uuid)
            .ok_or(AppError::MissingBoard { uuid })
    }

    /// Get a [`BoardStatus`] by [`Uuid`].
    pub fn get_board_status(&self, uuid: Uuid) -> Result<BoardStatus, AppError> {
        self.get_board(uuid).and_then(|lock| {
            lock.read()
                .map_err(|_| AppError::LockPoisoned("Board"))
                .and_then(|board| {
                    let status = board.status();

                    #[cfg(feature = "debug")]
                    {
                        let uuid = board.uuid();
                        logger::debug(&format!(
                            "Current board state for {uuid}:\n\u{1b}[39m{board}"
                        ));
                    }

                    // Override to accommodate "debug"
                    #[allow(clippy::let_and_return)]
                    status
                })
        })
    }

    /// Get all the [`StrikeReport`]s that occured on the specified board.
    pub fn get_board_strikes(&self, uuid: Uuid) -> Result<Vec<StrikeReport>, AppError> {
        self.get_board(uuid).and_then(|lock| {
            lock.read()
                .map_err(|_| AppError::LockPoisoned("Board"))
                .map(|board| board.strike_reports())
        })
    }

    /// List all the games.
    pub fn list_board_statuses(&self) -> Result<Vec<BoardStatus>, AppError> {
        Result::from_iter(self.boards.keys().map(|uuid| self.get_board_status(*uuid)))
    }

    /// Log visit.
    pub fn log_visit(&mut self, visit: PageVisit) {
        logger::debug(&format!(
            "Page Visit to {path} from {client} resulted in {code:03} {description}.",
            path = visit.path,
            client = visit
                .client
                .clone()
                .unwrap_or("<Unknown client>".to_owned()),
            code = visit
                .status_code
                .map(|status_code| status_code as u16)
                .unwrap_or(0),
            description = visit
                .status_code
                .as_ref()
                .map(tide::StatusCode::canonical_reason)
                .unwrap_or("(None)"),
        ));

        if !self.page_visits.contains_key(&visit.path) {
            self.page_visits.insert(visit.path.to_owned(), 1.into());
        } else {
            self.page_visits
                .get_mut(&visit.path)
                .map(|counter| counter.fetch_add(1, Ordering::Relaxed));
        }

        #[cfg(feature = "debug")]
        self.visit_log.push(visit)
    }
}
