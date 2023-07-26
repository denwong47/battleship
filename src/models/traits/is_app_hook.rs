use std::sync::{Arc, RwLock};

use crate::app::AppState;

pub trait IsAppHook {
    /// Create a new [`NewBoardHook`] from a [`AppState`] behind a [`RwLock`].
    fn new(locked_state: Arc<RwLock<AppState>>) -> Self;
}
