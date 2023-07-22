//! Stores the state of the app.
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use time::OffsetDateTime;
use uuid::Uuid;

use crate::{error::AppError, models::{Board, BoardStatus}};

use super::tasks::termination::TerminationToken;

#[cfg(feature="debug")]
use crate::logger;

pub struct AppState {
    start_time: OffsetDateTime,
    termination_token: Arc<TerminationToken>,
    boards: HashMap<Uuid, RwLock<Board>>,
}

impl Default for AppState {
    /// Create a default [`AppState`] using the current UTC time.
    fn default() -> Self {
        Self {
            start_time: OffsetDateTime::now_utc(),
            termination_token: Arc::new(TerminationToken::default()),
            boards: HashMap::new(),
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
    pub fn new_board(&mut self, size: [usize; 2], ship_count: u16) -> Result<BoardStatus, AppError> {
        let result = self.add_board(Board::new(size, ship_count));

        result
        .and_then(
            |uuid| {
                self.boards
                .get(&uuid)
                .ok_or_else(
                    || AppError::MissingBoard { uuid: uuid }
                )
                .and_then(
                    |board|
                        board
                        .read()
                        .map_err(
                            |_| AppError::LockPoisoned("Board")
                        )?
                        .status()
                )
            }
        )
    }

    /// Get a board by [`Uuid`].
    pub fn get_board<'s>(&'s self, uuid: Uuid) -> Result<&'s RwLock<Board>, AppError> {
        self.boards
        .get(&uuid)
        .ok_or_else(
            || AppError::MissingBoard { uuid }
        )
    }

    /// Get a [`BoardStatus`] by [`Uuid`].
    pub fn get_board_status(&self, uuid: Uuid) -> Result<BoardStatus, AppError> {
        self
        .get_board(uuid)
        .and_then(
            |lock|
                lock
                .read()
                .map_err(
                    |_| AppError::LockPoisoned("Board")
                )
                .and_then(
                    |board| {
                        let status = board.status();
                        
                        #[cfg(feature="debug")]
                        {
                            let uuid = board.uuid();
                            logger::debug(&format!("Current board state for {uuid}:\n\u{1b}[39m{board}"));
                        }

                        status
                    }
                )
        )
    }

    /// List all the games.
    pub fn list_board_statuses(&self) -> Result<Vec<BoardStatus>, AppError> {
        Result::from_iter(
            self
            .boards
            .keys()
            .map(|uuid| self.get_board_status(*uuid))
        )
    }
}
