use std::sync::{Arc, RwLock};

use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{traits::Segment, Board, Coordinates, Orientation, ShipStatus, ShipType, Strike};
use crate::{config, error::AppError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ship {
    pub uuid: Uuid,
    pub ship_type: ShipType,
    pub coordinates: Coordinates,
    orientation: Orientation,
    strikes: Vec<RwLock<Option<Arc<Strike>>>>,
}

impl PartialEq for Ship {
    /// Check if the two ships are the same.
    ///
    /// Only checks the internal uuid of the ships.
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Segment for Ship {
    /// Return the x position of the ship.
    fn x(&self) -> usize {
        self.coordinates.x
    }

    /// Return the y position of the ship.
    fn y(&self) -> usize {
        self.coordinates.y
    }

    /// Return the length of the ship.
    fn length(&self) -> usize {
        self.ship_type.length()
    }

    /// Return the orientation of the ship.
    fn orientation(&self) -> &Orientation {
        &self.orientation
    }
}

impl Ship {
    /// Get the identifier of a [`Strike`].
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    /// Return the current status of the ship.
    pub fn status(&self) -> Result<ShipStatus, AppError> {
        let (all_none, all_some) = self.strikes.iter().fold(Ok((true, true)), |result, lock| {
            lock.read()
                .map_err(|_| AppError::LockPoisoned("Ship strikes index"))
                .and_then(|opt| result.map(|(all_none, all_some)| (all_none, all_some, opt)))
                .map(|(mut all_none, mut all_some, opt)| {
                    all_none = all_none && opt.is_none();
                    all_some = all_some && opt.is_some();

                    (all_none, all_some)
                })
        })?;

        if all_none {
            Ok(ShipStatus::Undiscovered)
        } else if all_some {
            Ok(ShipStatus::Sunk)
        } else {
            Ok(ShipStatus::Discovered)
        }
    }

    /// Count the number of strikes this [`Ship`] had sustained.
    pub fn damages(&self) -> Result<usize, AppError> {
        let is_struck: Vec<bool> = Result::from_iter(self.strikes.iter().map(|lock| {
            lock.read()
                .map(|opt| opt.is_some())
                .map_err(|_| AppError::LockPoisoned("Ship strikes index"))
        }))?;

        // Count the number of cells that are struck.
        Ok(is_struck.into_iter().filter(|struck| *struck).count())
    }

    /// Randomly generate a ship.
    pub fn add_to_board(board: &mut Board) -> bool {
        (0..config::SHIP_GENERATION_MAX_RETRY).any(|_| {
            let mut rng = rand::thread_rng();

            let ship_type: ShipType = rand::random();
            let orientation: Orientation = rand::random();
            let coordinates: Coordinates = match &orientation {
                Orientation::Down => Coordinates::new(
                    rng.gen_range(0..board.width()),
                    rng.gen_range(0..board.height() - ship_type.length()),
                ),
                Orientation::Right => Coordinates::new(
                    rng.gen_range(0..board.width() - ship_type.length()),
                    rng.gen_range(0..board.height()),
                ),
            };

            // Initializes all the strikes.
            let mut strikes = Vec::with_capacity(ship_type.length());
            (0..ship_type.length()).for_each(|_| strikes.push(RwLock::new(None)));

            let ship = Self {
                uuid: Uuid::new_v4(),
                ship_type,
                coordinates,
                orientation,
                strikes,
            };

            board.add_ship(ship).is_ok()
        })
    }

    /// Attempts to strike the ship.
    pub fn strike(&self, strike: Arc<Strike>) -> Result<bool, AppError> {
        self.iter()
            .enumerate()
            .find(|(_, coordinates)| coordinates == &strike.coordinates)
            .map(|(id, _)| {
                // Store the successful strike into the ship.
                self.strikes
                    .get(id)
                    .unwrap() // assured above
                    .write()
                    .map(|mut container| {
                        *container = Some(strike);
                        true
                    })
                    .map_err(|_| AppError::LockPoisoned("Ship strikes index"))
            })
            .unwrap_or(Ok(false))
    }
}
