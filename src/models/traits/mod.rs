//! # Traits
//!
//! This module contains all the traits used by the models.
//!
mod add_default_headers;
pub use add_default_headers::*;

mod is_app_hook;
pub use is_app_hook::*;

mod query_params;
pub use query_params::*;

mod response_builder;
pub use response_builder::*;

mod segment;
pub use segment::*;
