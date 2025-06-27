pub mod cache;
pub mod conversions;
pub mod mappings;
pub mod unitstrings;

use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}
