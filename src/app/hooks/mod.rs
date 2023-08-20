//! Hooks to be added to the tide App.
mod app_status;
pub use app_status::*;

mod board_status;
pub use board_status::BoardStatusHook;

mod drop_board;
pub use drop_board::DropBoardHook;

mod list_boards;
pub use list_boards::ListBoardsHook;

mod list_strikes;
pub use list_strikes::ListStrikesHook;

mod new_board;
pub use new_board::NewBoardHook;

#[cfg(feature = "simulate_failures")]
mod simulated_failure;
#[cfg(feature = "simulate_failures")]
pub use simulated_failure::*;

mod strike;
pub use strike::StrikeHook;

mod termination;
pub use termination::TerminationHook;
