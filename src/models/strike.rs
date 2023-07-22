use super::Coordinates;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Recording a [`Strike`].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Strike {
    uuid: Uuid,
    pub coordinates: Coordinates,
}

impl PartialEq for Strike {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Strike {
    /// Get the identifier of a [`Strike`].
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    /// Instantiate a new [`Strike`] object.
    pub fn new(coordinates: Coordinates) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            coordinates,
        }
    }

    /// Return the x position of the strike.
    pub fn x(&self) -> usize {
        self.coordinates.x
    }

    /// Return the y position of the strike.
    pub fn y(&self) -> usize {
        self.coordinates.y
    }

    /// Return the coordinates of the strike.
    pub fn coordinates(&self) -> [usize; 2] {
        [self.x(), self.y()]
    }
}
