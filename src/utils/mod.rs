pub mod conversions;
pub mod mappings;
pub mod unitstrings;

use crate::Units;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

pub fn get_file_cache(t: &str, p: &str, u: Units) -> String {
    let dirs = xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME"));

    let mut hasher = DefaultHasher::new();
    let f = format!(
        "{}-{}",
        p,
        match u {
            Units::Metric => "metric",
            Units::Imperial => "imperial",
        }
    );
    f.hash(&mut hasher);

    let hash = format!("{:x}", hasher.finish());
    format!("{}{}-{}.cache", dirs.get_cache_home().unwrap_or_default().display(), t, hash)
}
