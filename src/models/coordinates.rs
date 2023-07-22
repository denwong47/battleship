use serde::{Deserialize, Serialize};

use super::Orientation;

/// Denotes the position of on a board.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}

impl Coordinates {
    /// Create a new [`Position`].
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Create a new [`Position`] by shifting rightwards.
    pub fn shift_right(&self, amount: i64) -> Self {
        Self::new((self.x as i64 + amount) as usize, self.y)
    }

    /// Create a new [`Position`] by shifting downwards.
    pub fn shift_down(&self, amount: i64) -> Self {
        Self::new(self.x, (self.y as i64 + amount) as usize)
    }

    /// Create a new [`Position`] by shifting.
    pub fn shift(&self, orientation: &Orientation, amount: i64) -> Self {
        match orientation {
            Orientation::Right => self.shift_right(amount),
            Orientation::Down => self.shift_down(amount),
        }
    }
}
