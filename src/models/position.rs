use std::{fmt::Display, sync::Arc};

use serde::{Deserialize, Serialize};

use super::{Ship, Strike};

pub const CHAR_EMPTY: char = '\u{a0}';
pub const CHAR_SHIP_UNHIT: char = '\u{2588}';
pub const CHAR_SHIP_HIT: char = '\u{2573}';
pub const CHAR_MISSED: char = '\u{2591}';

/// Indicates the state of a position on the grid.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Position {
    Empty,
    ShipUnhit(Arc<Ship>),
    ShipHit(Arc<Ship>, Arc<Strike>),
    Missed(Arc<Strike>),
}

impl Default for Position {
    fn default() -> Self {
        Self::Empty
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            Self::Empty => CHAR_EMPTY.to_string(),
            Self::ShipUnhit(ship) => ship.ship_type.colour_wraps(&CHAR_SHIP_UNHIT.to_string()),
            Self::ShipHit(ship, _) => ship.ship_type.colour_wraps(&CHAR_SHIP_HIT.to_string()),
            Self::Missed(_) => CHAR_MISSED.to_string(),
        };

        (0..=1).try_fold((), |_, _| write!(f, "{}", content))
    }
}
