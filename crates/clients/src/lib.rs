//! Exposes all interfaces with external systems, including arbiter simulations
//! and connections to live networks.

pub mod client;
pub mod dev;
pub mod forking;
pub mod ledger;
pub mod protocol;
pub mod scroll;

use ethers::prelude::*;

/// Import `TEST_SUBSCRIBER` into a test file to reference this already set
/// global tracer.
#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use tracing::Level;

    lazy_static! {
        pub static ref TEST_SUBSCRIBER: () = {
            // Get rust environment "RUST_TRACING_LEVEL" and set tracing level
            // accordingly.
            // Defaults to "Error".

            let _ = dotenv::dotenv();
            let tracing_level = std::env::var("RUST_TRACING_LEVEL")
                .unwrap_or_else(|_| "error".to_string())
                .parse::<Level>()
                .unwrap_or(Level::ERROR);

            let subscriber = tracing_subscriber::fmt()
                .with_max_level(tracing_level)
                .finish();
            tracing::subscriber::set_global_default(subscriber)
                .expect("Failed to set global default");
        };
    }
}
