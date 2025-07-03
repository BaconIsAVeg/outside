pub mod cache;
pub mod conversions;
pub mod mappings;
pub mod unitstrings;
pub mod urls;

use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the current Unix timestamp in seconds.
///
/// This function provides a consistent way to get the current time for
/// cache age calculations and timestamp comparisons throughout the application.
///
/// # Returns
///
/// Returns the number of seconds since the Unix epoch (1970-01-01 00:00:00 UTC).
///
/// # Panics
///
/// Panics if the system time is before the Unix epoch (extremely unlikely on modern systems).
pub fn get_now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}
