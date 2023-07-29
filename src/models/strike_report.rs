use lazy_static::__Deref;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;

use super::{Coordinates, Position, ShipIntel, Strike};

#[allow(unused_imports)]
use super::Board;

/// A report for a strike, formatted for export.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StrikeReport {
    uuid: Uuid,
    coordinates: Coordinates,
    hit: bool,
    intel: Option<ShipIntel>,
    ships_remaining: Option<usize>,
}

impl StrikeReport {
    /// Create a new [`StrikeReport`] from raw data.
    pub fn new(
        uuid: Uuid,
        coordinates: Coordinates,
        hit: bool,
        intel: Option<ShipIntel>,
        ships_remaining: Option<usize>,
    ) -> Self {
        Self {
            uuid,
            coordinates,
            hit,
            intel,
            ships_remaining,
        }
    }

    /// Try create a [`StrikeReport`] from a [`Coordinates`].
    pub fn try_from_position(value: &Position, repeated: bool) -> Result<Self, AppError> {
        let missed_factory = |strike: &Strike| {
            Self::new(strike.uuid(), strike.coordinates.clone(), false, None, None)
        };

        match value {
            Position::Missed(strike) => Ok(missed_factory(strike)),
            Position::ShipHit(ship, strike) => {
                if repeated {
                    Ok(missed_factory(strike))
                } else {
                    ShipIntel::try_from(ship.deref()).map(|intel| {
                        Self::new(
                            strike.uuid(),
                            strike.coordinates.clone(),
                            true,
                            Some(intel),
                            None,
                        )
                    })
                }
            }
            _ => Err(AppError::StrikeReportRequestedOnEmpty),
        }
    }

    /// Add a value to `ships_remaining`.
    ///
    /// This is used typically with `try_from_position`, since that method does
    /// not have access to the [`Board`] in order to obtain the number. This
    /// allows a chained method to add the number by the caller immediately
    /// after instantiation.
    pub fn with_ships_remaining(mut self, ships_remaining: usize) -> Self {
        self.ships_remaining = Some(ships_remaining);

        self
    }
}
