//! Defines orientation on the board.
use serde::{Deserialize, Serialize};

/// Orientation of the ship.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShipStatus {
    Undiscovered,
    Discovered,
    Sunk,
}
