//! # Trait: [`IsAppHook`]
//!
//! Marker trait for [`tide::Endpoint`]s that require access to an [`AppState`].

use std::sync::{Arc, RwLock};

use crate::app::AppState;

/// Marker trait for [`tide::Endpoint`]s that require access to an [`AppState`].
///
/// Contains only one method: [`IsAppHook::new()`] to create a new instance from
/// a [`AppState`] behind a [`RwLock`].
pub trait IsAppHook {
    /// Create a new instance from a [`AppState`] behind a [`RwLock`].
    fn new(locked_state: Arc<RwLock<AppState>>) -> Self;
}
