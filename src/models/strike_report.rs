use lazy_static::__Deref;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;

use super::{Coordinates, Position, ShipIntel, Strike};

/// A report for a strike, formatted for export.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StrikeReport {
    uuid: Uuid,
    coordinates: Coordinates,
    hit: bool,
    intel: Option<ShipIntel>,
}

impl StrikeReport {
    /// Create a new [`StrikeReport`] from raw data.
    pub fn new(uuid: Uuid, coordinates: Coordinates, hit: bool, intel: Option<ShipIntel>) -> Self {
        Self {
            uuid,
            coordinates,
            hit,
            intel,
        }
    }

    /// Try create a [`StrikeReport`] from a [`Position`].
    pub fn try_from_position(value: &Position, repeated: bool) -> Result<Self, AppError> {
        let missed_factory =
            |strike: &Strike| Self::new(strike.uuid(), strike.coordinates.clone(), false, None);

        match value {
            Position::Missed(strike) => Ok(missed_factory(strike)),
            Position::ShipHit(ship, strike) => {
                if repeated {
                    Ok(missed_factory(strike))
                } else {
                    ShipIntel::try_from(ship.deref()).map(|intel| {
                        Self::new(strike.uuid(), strike.coordinates.clone(), true, Some(intel))
                    })
                }
            }
            _ => Err(AppError::StrikeReportRequestedOnEmpty),
        }
    }
}
