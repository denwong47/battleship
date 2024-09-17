//! [`clap`] based CLI parameters parser, accepting command line override parameters.

use clap::Parser;
use std::path::PathBuf;

use crate::config;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(long_about = None)]
pub struct CommandLineParameters {
    /// Binds the host to a custom hosting address.
    #[arg(short, long, value_name = "HOST")]
    pub addr: Option<String>,

    /// Binds the host to this custom port.
    #[arg(short, long, value_name = "PORT")]
    pub port: Option<u16>,

    /// Sets a custom JSON config file. Defaults to `./host.json`.
    #[arg(short, long, value_name = "FILE", default_value = config::DEFAULT_CONFIG_PATH)]
    pub config: PathBuf,

    /// Sets the factor of which a request can fail. For instance, if this value is `32`, then the requests
    /// will fail on average 1 in 32 times.
    ///
    /// If `0`, no failures will be simulated even if `simulate_failures` feature is enabled.
    ///
    /// Only used if the binary is built with `simulate_failures` feature.
    #[arg(short, long, value_name = "int")]
    pub simulated_failure_factor: Option<usize>,
}
