//! Defines orientation on the board.
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use serde::{Deserialize, Serialize};

/// Orientation of the ship.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Orientation {
    Right,
    Down,
}

impl Distribution<Orientation> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Orientation {
        match rng.gen_range(0..=1) {
            0 => Orientation::Right,
            _ => Orientation::Down,
        }
    }
}
