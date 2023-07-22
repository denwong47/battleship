use lazy_static::lazy_static;
use time;

mod host_config;
pub use host_config::HostConfiguration;

/// Maximum number of retries for each ship before we give up.
pub const SHIP_GENERATION_MAX_RETRY: u16 = 32;

pub const DEFAULT_CONFIG_PATH: &str = "./host.json";
pub const DEFAULT_ADDR: &str = "0.0.0.0";
pub const DEFAULT_PORT: u16 = 8080;

pub const DEFAULT_BOARD_SIZE: [usize; 2] = [16, 16];
pub const DEFAULT_SHIP_COUNT: u16 = 8;

lazy_static!(
    /// The default datetime format for use in this app.
    ///
    /// This shortened format is compatible with Python's `datetime.fromisoformat`.
    pub static ref DATETIME_FORMAT: &'static [time::format_description::FormatItem<'static>] =
        time::macros::format_description!(
            "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:6]"
        )
    ;
);
