//! A `host.json` file is required at the current working directory where the host is
//! running from.
//!
//! A typical `host.json`:
//! ```
//! {
//!     "addr": "0.0.0.0",
//!     "port": 8080,
//!     "simulated_failure_factor": 32
//! }
//! ```
//!
//! Available keys:
//!
//! - `addr` - IP address or hostname the server should bind to. Typically this should
//!   be `0.0.0.0`, allowing the host to be accessible on all addresses; however this can
//!   be changed to the IP address of a specific interface, allowing it exclusive access,
//!   or it can be `localhost`, permitting local access only.
//! - `port` - port number the server should bind to. Must be within `1` and `65535`; if the port is already in use, an error is emitted:
//!   ```text
//!   Terminating `battleship` server with error: App at 0.0.0.0:8080 failed to start up:
//!   Address already in use (os error 48)
//!   ```
//!   And the app will terminate immediately.
//! - `simulated_failure_factor` - the server may simulate a `502 Gateway Timeout` or a
//!   `408 Request Timeout` error every `X` requests on average, where `X` is the value
//!   specified here.
//!  
//!   If `0`, this feature is disabled.
