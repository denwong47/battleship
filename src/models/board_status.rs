use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;

use super::{Board, ShipIntel};

#[cfg(feature = "debug")]
use std::sync::Arc;

#[cfg(feature = "debug")]
use super::Ship;

/// A brief summary of the status of the board.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BoardStatus {
    uuid: Uuid,
    size: [usize; 2],
    active: bool,
    elapsed: f64,
    strikes: usize,
    ship_intel: Vec<ShipIntel>,
    #[cfg(feature = "debug")]
    ship_status: Vec<Arc<Ship>>,
    #[cfg(feature = "debug")]
    display: String,
}

impl TryFrom<&Board> for BoardStatus {
    type Error = AppError;

    fn try_from(value: &Board) -> Result<Self, Self::Error> {
        Ok(Self {
            uuid: value.uuid(),
            size: [value.width(), value.height()],
            active: !value.is_frozen(),
            elapsed: value.elapsed().as_seconds_f64(),
            strikes: value.strike_count(),
            ship_intel: value.ship_intel()?,

            #[cfg(feature = "debug")]
            ship_status: value.ships(),

            #[cfg(feature = "debug")]
            display: value.to_string(),
        })
    }
}
