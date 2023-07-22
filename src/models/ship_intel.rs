use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::AppError,
    models::{Ship, ShipStatus, ShipType},
};

#[allow(unused_imports)]
use crate::models::Board;

/// An abridged status of a [`Ship`] for the purpose of returning a global view of the ships on a [`Board`].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShipIntel {
    uuid: Uuid,
    ship_type: ShipType,
    status: ShipStatus,
    damages: usize,
}

impl TryFrom<&Ship> for ShipIntel {
    type Error = AppError;
    fn try_from(value: &Ship) -> Result<Self, Self::Error> {
        Ok(Self {
            uuid: value.uuid,
            ship_type: value.ship_type.clone(),
            status: value.status()?,
            damages: value.damages()?,
        })
    }
}
