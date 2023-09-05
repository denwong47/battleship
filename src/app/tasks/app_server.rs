use std::sync::{Arc, RwLock};

use tide::Server;

use crate::{
    app::{hooks, AppHook, AppState},
    logger,
};

/// Create the app an populate it with the routes we need.
pub async fn create_app(app_state: AppState) -> Server<()> {
    logger::debug("Initialising `tide::Server`...");

    let locked_state = Arc::new(RwLock::new(app_state));

    let mut app = tide::new();

    macro_rules! expand_paths {
        (
            $((
                $path:literal, $hook:ident
            )),+$(,)?
        ) => {
            $(
                app.at($path)
                    .get(AppHook::<hooks::$hook>::new(Arc::clone(&locked_state)));
            )*
        };
    }

    expand_paths!(
        ("/info", AppStatusHook),
        ("/list", ListBoardsHook),
        ("/terminate", TerminationHook),
        ("/new", NewBoardHook),
        ("/status/:uuid", BoardStatusHook),
        ("/list_strikes/:uuid", ListStrikesHook),
        ("/strike/:uuid", StrikeHook),
        ("/drop/:uuid", DropBoardHook),
    );

    logger::debug("`tide::Server` created...");
    app
}
