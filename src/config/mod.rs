//! Configuration module, containing constants, statics and [`HostConfiguration`] struct
//! to store runtime configurations.

use lazy_static::lazy_static;
use time;

mod host_config;
pub use host_config::HostConfiguration;

/// Maximum number of retries for each ship before we give up.
pub const SHIP_GENERATION_MAX_RETRY: u16 = 32;

pub const DEFAULT_CONFIG_PATH: &str = "./host.json";
pub const DEFAULT_ADDR: &str = "0.0.0.0";
pub const DEFAULT_PORT: u16 = 8080;
pub const DEFAULT_SIMULATED_FAILURE_FACTOR: usize = 32;

pub const DEFAULT_BOARD_SIZE: [usize; 2] = [16, 16];
pub const DEFAULT_SHIP_COUNT: u16 = 8;

lazy_static!(
    /// The default datetime format for use in this app.
    ///
    /// This shortened format is compatible with Python's `datetime.fromisoformat`.
    pub static ref DATETIME_FORMAT: &'static [time::format_description::FormatItem<'static>] =
        time::macros::format_description!("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:6]")
    ;
);

time::serde::format_description!(
    _serde_offset_date_time,
    OffsetDateTime,
    "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:6]"
);

/// Serialize and Deserialize module for `DATETIME_FORMAT`.
///
/// # Example
///
/// ```rust
/// use serde::{Serialize, Deserialize};
/// use time::OffsetDateTime;
/// use battleship::config::serde_offset_date_time;
///
/// #[derive(Serialize, Deserialize)]
/// pub struct MyStruct {
///     #[serde(with="serde_offset_date_time")]
///     timestamp: OffsetDateTime,
/// }
/// ```
pub mod serde_offset_date_time {
    pub use super::_serde_offset_date_time::*;
}
