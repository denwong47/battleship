//! A collection of structs and traits to facilitate the game.
//!
//! #### Note
//!
//! Objects in this module should be considered useful for export; thus app specific
//! structs such as those implementing [Endpoint] should not be in this module.
//!
//! [Endpoint]: `tide::Endpoint`

mod board;
pub use board::*;

mod board_status;
pub use board_status::*;

mod coordinates;
pub use coordinates::*;

mod orientation;
pub use orientation::*;

mod position;
pub use position::*;

mod ship;
pub use ship::*;

mod ship_intel;
pub use ship_intel::*;

mod ship_status;
pub use ship_status::*;

mod ship_types;
pub use ship_types::*;

mod strike;
pub use strike::*;

mod strike_report;
pub use strike_report::*;

pub mod traits;

#[allow(unused_imports)]
use tide::Endpoint;
