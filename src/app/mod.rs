//! [`tokio`] application, including the main [`tide`] routine to host the game.
//!
//! ## Requirements
//!
//! A [`host.json`] file must be setup in the current working directory from which the app is executed.
//!
//! [`host.json`]: `crate::_doc::hostjson`
//!
//! ## API Documentation
//!
//! The following [`tide::Endpoint`]s are available:
//!
//! - Query for the server status
//!     - [info] - logged statistics and server parameters.
//!     - [list] - list all the [board status]es available on this host.
//!     - [terminate] - terminate the host immediately, losing all the active boards in the process.
//! - Game board related queries
//!     - [new] - create a new game [`Board`].
//!     - [status] - [board status] of a single existing [`Board`].
//!     - [list_strikes] - list all the recorded [`Strike`]s on an existing board.
//!     - [strike] - perform a [`Strike`] at a designated coordinates on an existing board.
//!     - [drop] - remove a [`Board`] from the host, and return the complete state of
//!       the board, including hidden ships etc.
//!
//! [list]: #list
//! [info]: #info
//! [terminate]: #terminate
//! [new]: #new
//! [status]: #statusuuid
//! [list_strikes]: #list_strikesuuid
//! [strike]: #strikeuuid
//! [drop]: #dropuuid
//!
//! [board status]: `BoardStatus`
//! [ship intelligence]: `ShipIntel`
//!

//! # Error Handling
//!
//! In general, all endpoints can return a JSON object indicating any errors it encountered during the query.
//! These object will take the format of:
//!
//! ```
//! {
//!     "error": "TypeOfError",
//!     "message": "A message explaining the error."
//! }
//! ```
//!
//! A non `200 OK` status will be returned alongside such a message.
//! Consult [`AppError`] for a complete list of possible error types and their
//! corresponding status codes.
//!

//! # `/info`
//!
//! **Prints out information about the state of the app.**
//!
//! This contains:
//! - `page_visits` - a mapping of each distinct endpoint to the number of visits they received. The end point path excludes any query parameters.
//! - `simulated_failure_factor` - the same value given in [`host.json`], loaded upon initiation of host.
//! - `start_time` - the time that the host began serving requests, in the format `YYYY-MM-DDTHH:mm:ss.nnnnnn`.
//!
//! If the executable is built with the `debug` feature, a full visit log will also
//! be included.
//!
//! #### Syntax
//! ```text
//! /info
//! ```
//!
//! #### Parameters
//!
//! This endpoint does not accept parameters.
//!
//! #### Example return
//!
//! ```
//! {
//!   "page_visits": {
//!     "/drop/55a8e295-909c-4054-a18f-87ef6002e343": 1,
//!     "/info": 2,
//!     "/new": 2,
//!     "/status/55a8e295-909c-4054-a18f-87ef6002e343": 1,
//!     "/status/v": 1,
//!     "/strike/55a8e295-909c-4054-a18f-87ef6002e343": 6
//!   },
//!   "simulated_failure_factor": 32,
//!   "start_time": "2023-08-01T21:09:31.404040"
//! }
//! ```
//!

//! # `/list`
//!
//! **List all the stored [board status]es on this host.**
//!
//! An unordered array of [board status]es in the same format as those returned by [status].
//!
//! #### Syntax
//! ```text
//! /list
//! ```
//!
//! #### Parameters
//!
//! This endpoint does not accept parameters.
//!
//! #### See also
//! - [status] for format of each returned item in the array.
//!
//! # `/terminate`
//!
//! **Immediately shut down the host gracefully, dropping all boards in the process.**
//!
//! #### Syntax
//! ```text
//! /terminate
//! ```
//!
//! This is only used for remote termination of the host. It has the same effect as
//! pressing `Ctrl-C` on the terminal running the app.
//!
//! While [`Board`] supports [`Serialize`] and [`Deserialize`], this is currently not being used by the host,
//! thus none of the existing [`Board`]s will be preserved; they will simply be dropped from memory.
//!
//! #### Parameters
//!
//! This endpoint does not accept parameters.
//!
//! #### Example return
//!
//! ```
//! {
//!     "action": "termination",
//!     "success": true
//! }
//! ```

//! # `/new`
//!
//! **Create a new game with the stated parameters.**
//!
//! Upon successful creation, returns the [board status] of the newly crated board.
//!
//! Use the key `uuid` to get the [`Board`] identifier.
//!
//! #### Syntax
//! ```text
//! /new?width=<u64>&height=<u64>&ship_count=<usize>
//! ```
//!
//! #### Parameters
//!
//! - `width`: board width, default `10`,
//! - `height`: board height, default `10`, and
//! - `ship_count`: number of ships to aim to create.
//!
//!   There is no guarantee that
//!   this amount of ships will be present in the resultant board - this was simply
//!   the aimed amount. Each ship creation is tried for `32` times before giving up.
//!   The number of ships will never exceed this number.
//!
//! #### Example return
//!
//! ```
//! {
//!   "action": "createGame",
//!   "game": {
//!     "active": true,
//!     "elapsed": 0.000089,
//!     "ship_intel": [
//!       {
//!         "damages": 0,
//!         "ship_type": "Battleship",
//!         "status": "Undiscovered",
//!         "uuid": "593f093a-620b-4bf4-ba31-e8ee5b4d9de5"
//!       },
//!       {
//!         "damages": 0,
//!         "ship_type": "Cruiser",
//!         "status": "Undiscovered",
//!         "uuid": "01e96180-2910-4f09-930d-f75bafcacf53"
//!       },
//!       {
//!         "damages": 0,
//!         "ship_type": "AircraftCarrier",
//!         "status": "Undiscovered",
//!         "uuid": "919c5d58-aabf-481e-ae7f-3f601816f629"
//!       },
//!       {
//!         "damages": 0,
//!         "ship_type": "Battleship",
//!         "status": "Undiscovered",
//!         "uuid": "4d22ea2a-5ec3-4117-a8e5-ed6ecae8efcf"
//!       }
//!     ],
//!     "size": [
//!       8,
//!       6
//!     ],
//!     "strikes": 0,
//!     "uuid": "9e3bfde9-4248-49fd-8550-3e773c3deca6"
//!   },
//!   "success": true
//! }
//! ```
//!

//! # `/status/<uuid>`
//!
//! **Return the [board status] without giving away any secrets of the game.**
//!
//! The following keys are returned:
//!
//! - `active` - `false` if the game is finished; otherwise `true`.
//! - `elapsed` - seconds since the game has started.
//! - `ship_intel` - an array of [ship intelligence] objects indicating what ships are
//!   present, their statuses and damages sustained.
//! - `size` - an array of size `2` indicating the board size: `[width: usize, height: usize]`
//! - `strikes` - number of strikes that had occurred.
//! - `uuid` - the unique identifier of the board.
//!
//! #### Parameters
//!
//! This endpoint does not accept parameters.
//!
//! #### Syntax
//!
//! ```text
//! /status/<uuid>
//! ```
//!
//! #### Example return
//!
//! ```
//! {
//!   "active": true,
//!   "elapsed": 991.084999,
//!   "ship_intel": [
//!     {
//!       "damages": 0,
//!       "ship_type": "Battleship",
//!       "status": "Undiscovered",
//!       "uuid": "593f093a-620b-4bf4-ba31-e8ee5b4d9de5"
//!     },
//!     {
//!       "damages": 0,
//!       "ship_type": "Cruiser",
//!       "status": "Undiscovered",
//!       "uuid": "01e96180-2910-4f09-930d-f75bafcacf53"
//!     },
//!     {
//!       "damages": 0,
//!       "ship_type": "AircraftCarrier",
//!       "status": "Undiscovered",
//!       "uuid": "919c5d58-aabf-481e-ae7f-3f601816f629"
//!     },
//!     {
//!       "damages": 0,
//!       "ship_type": "Battleship",
//!       "status": "Undiscovered",
//!       "uuid": "4d22ea2a-5ec3-4117-a8e5-ed6ecae8efcf"
//!     }
//!   ],
//!   "size": [
//!     8,
//!     6
//!   ],
//!   "strikes": 0,
//!   "uuid": "9e3bfde9-4248-49fd-8550-3e773c3deca6"
//! }
//! ```

//! # `/list_strikes/<uuid>`
//!
//! **List all the strikes that had occurred on this board so far.**
//!
//! This is not an accurate "replay" of all the [`Strike`]s. It differs by two ways:
//!
//! - the [`ShipIntel`] will always reflect the current state of the ship, not when the
//!   [`Strike`] occurred.
//! - the `ships_remaining` field is not populated; it will always be `null`.
//!
//! A typical use of this [`Endpoint`] is for a client to render the state of a
//! [`Board`] without any prior knowledge of what happened. In such a scenario, tracing
//! the history and progress of the board accurately is not required.
//!
//! #### Parameters
//!
//! This endpoint does not accept parameters.
//!
//! #### Syntax
//!
//! ```text
//! /list_strikes/<uuid>
//! ```

//! # `/strike/<uuid>`
//!
//! #### Syntax
//!
//! ```text
//! /strike/<uuid>?x=<u64>&y=<u64>
//! ```
//!
//! **Perform a strike on the specified coordinates.**
//!
//! #### Example return
//!
//! - Missed:
//!
//!   ```
//!   {
//!     "coordinates": {
//!       "x": 12,
//!       "y": 4
//!     },
//!     "hit": false,
//!     "intel": null,
//!     "ships_remaining": 7,
//!     "uuid": "281834dc-a688-45a2-ba72-311a757cc0ca"
//!   }
//!   ```
//! - Hit:
//!
//!   ```
//!   {
//!     "coordinates": {
//!       "x": 7,
//!       "y": 9
//!     },
//!     "hit": true,
//!     "intel": {
//!       "damages": 1,
//!       "ship_type": "Battleship",
//!       "status": "Discovered",
//!       "uuid": "91efacf4-f97d-4478-b1aa-a7df6d2534f7"
//!     },
//!     "ships_remaining": 7,
//!     "uuid": "af7d69e9-1809-4d1e-97aa-d76ce3fa303e"
//!   }
//!   ```
//! - Coordinates out of bounds:
//!
//!   ```
//!   {
//!     "error": "CoordinatesOutOfBounds",
//!     "message": "Coordinates (7, 16) is out of bounds."
//!   }
//!   ```
//!

//! # `/drop/<uuid>`
//!
//! #### Syntax
//!
//! ```text
//! /drop/<uuid>
//! ```
//!
//! **Finish a game, and return the current unobscured state of the board.**
//!
//! This will remove the game from the server permanently.
//!
//! #### Example return
//!
//! ```
//! {
//!   "frozen": false,
//!   "position_index": {
//!     ...
//!   },
//!   "ships": [
//!     {
//!       "coordinates": {
//!         "x": 1,
//!         "y": 2
//!       },
//!       "orientation": "Down",
//!       "ship_type": "Cruiser",
//!       "strikes": [
//!         null,
//!         null,
//!         null
//!       ],
//!       "uuid": "c79d876c-2778-4aca-ac6d-f643b14e2b14"
//!     },
//!     {
//!       "coordinates": {
//!         "x": 0,
//!         "y": 1
//!       },
//!       "orientation": "Right",
//!       "ship_type": "Submarine",
//!       "strikes": [
//!         null,
//!         null,
//!         null
//!       ],
//!       "uuid": "150024d8-53c3-4b58-9e13-d11380b47885"
//!     },
//!     {
//!       "coordinates": {
//!         "x": 2,
//!         "y": 2
//!       },
//!       "orientation": "Down",
//!       "ship_type": "Cruiser",
//!       "strikes": [
//!         null,
//!         null,
//!         null
//!       ],
//!       "uuid": "f34f6cd3-e9f7-4b4a-b448-eee830941fc4"
//!     }
//!   ],
//!   "size": [
//!     6,
//!     6
//!   ],
//!   "start_time": "2023-07-31T08:20:51.814026",
//!   "strikes": [],
//!   "uuid": "255f3c2b-b5dd-4850-b0e9-bfc6551747b4"
//! }
//! ```
//!
mod base;
pub use base::run_app;

mod app_hook;
pub use app_hook::*;

mod app_state;
pub use app_state::*;

pub mod hooks;

mod page_visit;
pub use page_visit::*;

pub mod tasks;

#[allow(unused_imports)]
use tide::Endpoint;

#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use crate::{error::AppError, models::*};
