//! Hooks to be added to the tide App.
mod board_status;
pub use board_status::BoardStatusHook;

mod list_boards;
pub use list_boards::ListBoardsHook;

mod new_board;
pub use new_board::NewBoardHook;

mod strike;
pub use strike::StrikeHook;

mod termination;
pub use termination::TerminationHook;
