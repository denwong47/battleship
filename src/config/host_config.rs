use std::{fs::File, io::BufReader, path::Path};

use serde::{Deserialize, Serialize};
use serde_json;

use crate::{args::CommandLineParameters, error::AppError};

use super::{DEFAULT_ADDR, DEFAULT_PORT, DEFAULT_SIMULATED_FAILURE_FACTOR};

/// A struct to parse the configurations from `host.json`, or whatever path `config` was
/// set to in the command line arguments.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct HostConfiguration {
    /// Binds the host to a hosting address.
    pub addr: Option<String>,

    /// Binds the host to this port.
    pub port: Option<u16>,

    /// Sets the factor of which a request can fail. For instance, if this value is `32`, then the requests
    /// will fail on average 1 in 32 times.
    ///
    /// If `0`, no failures will be simulated even if `simulate_failures` feature is enabled.
    ///
    /// Only used if the binary is built with `simulate_failures` feature.
    pub simulated_failure_factor: Option<usize>,
}

impl HostConfiguration {
    pub fn try_from_file<P>(path: P) -> Result<Self, AppError>
    where
        P: AsRef<Path>,
    {
        let file = File::open(&path).map_err(|err| AppError::ConfigNotReadable {
            path: AsRef::<Path>::as_ref(&path).display().to_string(),
            err,
        })?;
        let reader = BufReader::new(file);

        serde_json::from_reader(reader).map_err(|err| AppError::ConfigNotParsable {
            path: AsRef::<Path>::as_ref(&path).display().to_string(),
            err,
        })
    }

    pub fn resolve_with_args(mut self, args: &CommandLineParameters) -> Self {
        self.addr = args
            .addr
            .as_ref()
            .or(self.addr.as_ref())
            .map(|s| s.to_owned())
            .or(Some(DEFAULT_ADDR.to_owned()));

        self.port = args.port.or(self.port).or(Some(DEFAULT_PORT));

        self.simulated_failure_factor = args
            .simulated_failure_factor
            .or(self.simulated_failure_factor)
            .or(Some(DEFAULT_SIMULATED_FAILURE_FACTOR));

        self
    }

    pub fn try_from_args(value: &CommandLineParameters) -> Result<Self, AppError> {
        Self::try_from_file(&value.config).map(|config| config.resolve_with_args(value))
    }
}
