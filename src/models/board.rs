use ndarray::{s, Array2, Ix2, SliceInfo, SliceInfoElem};
use serde::{Deserialize, Serialize};
use std::{
    ops::Deref,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use time::OffsetDateTime;
use uuid::Uuid;

use super::{
    traits::Segment, BoardStatus, Coordinates, Orientation, Position, Ship, ShipIntel, ShipStatus,
    Strike, StrikeReport,
};

use crate::{config, error::AppError};

#[cfg(feature = "debug")]
use std::fmt::Display;

#[cfg(feature = "debug")]
use crate::logger;

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    uuid: Uuid,
    #[serde(with = "config::serde_offset_date_time")]
    start_time: OffsetDateTime,
    size: [usize; 2],
    ships: Vec<Arc<Ship>>,
    strikes: Vec<Arc<Strike>>,
    position_index: Array2<Position>,
    frozen: AtomicBool,
}

impl Board {
    /// Get the identifier of a [`Strike`].
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    /// Create a new [`Board`] instance.
    pub fn new(size: [usize; 2], ship_count: u16) -> Self {
        #[cfg(feature = "debug")]
        logger::debug(&format!(
            "Creating a new board of size {w}x{h}, with {count} ships.",
            w = size[0],
            h = size[1],
            count = &ship_count,
        ));

        let board = Self {
            uuid: Uuid::new_v4(),
            start_time: OffsetDateTime::now_utc(),
            size,
            ships: Vec::with_capacity(ship_count as usize),
            strikes: Vec::new(),
            position_index: Array2::default(size),
            frozen: false.into(),
        };

        (0..ship_count).fold(
            board,
            #[allow(unused_variables)]
            |mut board, id| {
                let success = Ship::add_to_board(&mut board);

                #[cfg(feature = "debug")]
                if success {
                    logger::debug(&format!("Successfully created ship #{id}.", id = &id,));
                } else {
                    #[cfg(feature = "debug")]
                    logger::error(&format!(
                        "Failed to create ship #{id}, board may be full.",
                        id = &id,
                    ));
                }

                board
            },
        )
    }

    /// The [`time::Duration`] that had elapsed since the start of the app.
    pub fn elapsed(&self) -> time::Duration {
        OffsetDateTime::now_utc() - self.start_time
    }

    /// The width of this board.
    pub fn width(&self) -> usize {
        self.size[0]
    }

    /// The height of this board.
    pub fn height(&self) -> usize {
        self.size[1]
    }

    /// Count the number of [`Strike`]s on this [`Board`].
    pub fn strike_count(&self) -> usize {
        self.strikes.len()
    }

    /// Return a public [`BoardStatus`] for this board.
    pub fn status(&self) -> Result<BoardStatus, AppError> {
        BoardStatus::try_from(self)
    }

    /// Freeze this [`Board`].
    pub fn freeze(&self) {
        self.frozen.store(true, Ordering::Relaxed);
    }

    /// Return if this [`Board`] is frozen.
    pub fn is_frozen(&self) -> bool {
        self.frozen.load(Ordering::Relaxed)
    }

    /// Return an [`Err`] if frozen.
    pub fn err_if_frozen(&self) -> Result<(), AppError> {
        if self.is_frozen() {
            Err(AppError::FrozenBoard { uuid: self.uuid() })
        } else {
            Ok(())
        }
    }

    /// Return the number of ships remaining.
    pub fn ships_remaining(&self) -> Result<usize, AppError> {
        self.ships.iter().fold(Ok(0), |count, ship| {
            count.and_then(|count| {
                Ok(count + {
                    if ship.status()? != ShipStatus::Sunk {
                        1
                    } else {
                        0
                    }
                })
            })
        })
    }

    /// Check if the player has won.
    pub fn has_won(&self) -> Result<bool, AppError> {
        // This uses the logic above, but short circuits when the first unsunk
        // ship is found.
        self.ships.iter().fold(Ok(true), |won, ship| {
            won.and_then(|won| Ok(won && (ship.status()? == ShipStatus::Sunk)))
        })
    }

    /// Check the availability of the positions specified.
    fn is_available(
        &self,
        coordinates: &Coordinates,
        orientation: &Orientation,
        length: usize,
    ) -> bool {
        (0..length).all(|point| {
            let shifted_coordinates = coordinates.shift(orientation, point as i64);

            #[cfg(feature = "debug")]
            logger::trace(&format!(
                "Checking {coordinates:?}...",
                coordinates = &shifted_coordinates,
            ));

            self.position_index
                .get([shifted_coordinates.x, shifted_coordinates.y])
                .map(|pos| pos == &Position::Empty)
                .unwrap_or(false)
        })
    }

    /// Set a slice of the board to a specific state.
    fn set_slice_state(&mut self, slice: SliceInfo<[SliceInfoElem; 2], Ix2, Ix2>, state: Position) {
        self.position_index.slice_mut(slice).fill(state)
    }

    /// Set state of some coordinates on the board to a specific state.
    fn set_segment_state(
        &mut self,
        coordinates: &Coordinates,
        orientation: &Orientation,
        length: usize,
        state: Position,
    ) {
        let slice = match orientation {
            Orientation::Down => s![
                coordinates.x..coordinates.x + 1,
                coordinates.y..(coordinates.y + length)
            ],
            Orientation::Right => s![
                coordinates.x..(coordinates.x + length),
                coordinates.y..coordinates.y + 1
            ],
        };

        self.set_slice_state(slice, state)
    }

    /// Takes ownership of a ship, and attempt to add it to the [`Board`].
    pub fn add_ship(&mut self, ship: Ship) -> Result<(), AppError> {
        self.err_if_frozen()?;

        let ship_arc = Arc::new(ship);

        // Since we already holds the mutable reference to `self`,
        // we don't need to confirm atomicity between `is_available` and
        // `set_segment_state`.
        self.is_available(
            &ship_arc.coordinates,
            ship_arc.orientation(),
            ship_arc.length(),
        )
        .then(|| {
            self.set_segment_state(
                &ship_arc.coordinates,
                ship_arc.orientation(),
                ship_arc.length(),
                Position::ShipUnhit(Arc::clone(&ship_arc)),
            );

            self.ships.push(Arc::clone(&ship_arc));
        })
        .ok_or_else(|| AppError::PositionOccupied {
            x: ship_arc.x(),
            y: ship_arc.y(),
        })
    }

    /// Takes ownership of a [`Strike`], then perform the strike onto the board.
    ///
    /// Repeated strikes will be logged, but the state of the [`Coordinates`] won't change.
    pub fn add_strike(&mut self, strike: Strike) -> Result<StrikeReport, AppError> {
        self.err_if_frozen()?;

        let strike_arc = Arc::new(strike);
        let index = strike_arc.coordinates();

        self.strikes.push(Arc::clone(&strike_arc));

        let result = if let Some(pos) = self.position_index.get_mut(index) {
            match pos {
                Position::Empty => {
                    *pos = Position::Missed(Arc::clone(&strike_arc));

                    StrikeReport::try_from_position(pos, false)
                }
                Position::ShipUnhit(ship) => {
                    ship.strike(Arc::clone(&strike_arc))?;
                    *pos = Position::ShipHit(Arc::clone(ship), Arc::clone(&strike_arc));

                    StrikeReport::try_from_position(pos, false)
                }
                _ => {
                    // If its already hit, then `repeated` is `true`. Otherwise its going to `Err`, doesn't matter.
                    StrikeReport::try_from_position(pos, true)
                }
            }
            // Add the number of ships remaining to the generated report
            .and_then(|report| Ok(report.with_ships_remaining(self.ships_remaining()?)))
        } else {
            Err(AppError::CoordinatesOutOfBounds {
                x: strike_arc.x(),
                y: strike_arc.y(),
            })
        };

        // Check if the ships had been blown up.
        if self.has_won()? {
            self.freeze();
        }

        result
    }

    /// Returns a report of all the ships statuses without giving away their positions.
    pub fn ship_intel(&self) -> Result<Vec<ShipIntel>, AppError> {
        Result::from_iter(
            self.ships
                .iter()
                .map(|ship| ShipIntel::try_from(ship.deref())),
        )
    }
}

#[cfg(feature = "debug")]
impl Board {
    /// Get a list of all ships.
    pub fn ships(&self) -> Vec<Arc<Ship>> {
        self.ships.clone()
    }
}

#[cfg(feature = "debug")]
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "   ")?;
        let _: Vec<()> =
            Result::from_iter((0..self.width()).map(|id| write!(f, "{:>2}", id % 10)))?;

        writeln!(f)?;

        let _: Vec<()> =
            Result::from_iter(self.position_index.columns().into_iter().enumerate().map(
                |(row_id, array)| {
                    writeln!(
                        f,
                        "{:2} {}",
                        row_id,
                        array
                            .iter()
                            .fold(String::new(), |string, item| { string + &item.to_string() })
                    )
                },
            ))?;

        Ok(())
    }
}
