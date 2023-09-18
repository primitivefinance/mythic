use arbiter_core::middleware::RevmMiddleware;
use std::sync::Arc;

/// Used to label an admin `Client` the simulation is running with.
pub const ADMIN_LABEL: &str = "admin";

/// Used to label the arbitrageur `Client` the simulation is running with.
pub const ARBITRAGEUR_LABEL: &str = "arbitrageur";

// The following constants are used throughout the simulation.
/// The number of seconds in a year.
pub const SECONDS_PER_YEAR: u64 = 31556953;
/// The number 10^18.
pub const WAD: ethers::types::U256 = ethers::types::U256([10_u64.pow(18), 0, 0, 0]);

pub type Client = Arc<RevmMiddleware>;
