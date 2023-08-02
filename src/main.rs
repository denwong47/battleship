//! # Battleship game
//! #### Single Player game through a self hosted API End point
//!
//! ![CI status](https://github.com/denwong47/battleship/actions/workflows/CI.yml/badge.svg?branch=main)
//! ![Github Pages](https://github.com/denwong47/battleship/actions/workflows/gh_pages.yml/badge.svg?branch=main)
//! ![Publish status](https://github.com/denwong47/battleship/actions/workflows/publish.yml/badge.svg?branch=main)
//!
//! This simple API End point hosts a single-player variation of the "Battleship"
//! game, in which a player call "shots" at the host's fleet of warships randomly
//! distributed on a concealed gridded board, with the objective of destroying all
//! of them using the least number of strikes.
//!
//! ![Hasbro version of the game](https://m.media-amazon.com/images/I/71GcJTrP0HL._AC_SL1500_.jpg)
//!
//! In contrast to the traditional board game, this API only has one board held by
//! the host. The host does not strike back at the player.
//!
//! ## What's the point of this?
//!
//! This was written to help a friend of mine understand ETL processes involving API
//! requests, hopefully in a somewhat meaningful and purposeful context. For this
//! reason the host includes "silly" features such as simulated network failures
//! (e.g. timeout).
//!
//! The game itself serves no purpose and is not necessarily entertaining. The true
//! purpose is to algorithmically solve the game via the API hooks, which records
//! statistics that cannot be easily cheated.

pub mod app;
pub mod args;
pub mod config;
pub mod error;
pub mod logger;
pub mod models;

use crate::error::AppError;
use clap::Parser;

/// The main handler for the whole app. Parses the CLI arguments, reads the
/// configuration file, then starts up [`app::run_app`] with the compiled configuration.
#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Resolve configurations
    let cli_args = args::CommandLineParameters::parse();
    let config = config::HostConfiguration::try_from_args(&cli_args)
        .unwrap_or_default()
        .resolve_with_args(&cli_args);

    logger::info("Starting `battleship` server.");

    let result = app::run_app(config).await;

    match &result {
        Ok(()) => logger::info("Terminating `battleship` server gracefully."),
        Err(err) => logger::error(&format!(
            "Terminating `battleship` server with error: {err}"
        )),
    }

    result
}
