//! Stores the state of the app.
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
    models::{Board, BoardStatus},
};

use super::{tasks::termination::TerminationToken, PageVisit};

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
        }
    }
}

impl AppState {
    /// Create a default [`AppState`] using the current UTC time.
    pub fn new() -> Self {
        Self::default()
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
