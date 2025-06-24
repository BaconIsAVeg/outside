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

// TODO: Move this to a more appropriate place
pub fn get_cached_file(datatype: &str, content: &str, u: Units) -> String {
    let mut hasher = DefaultHasher::new();
    let f = format!(
        "{}-{}",
        content,
        match u {
            Units::Metric => "metric",
            Units::Imperial => "imperial",
        }
    );
    f.hash(&mut hasher);

    let hash = format!("{:x}", hasher.finish());

    std::fs::create_dir_all(
        dirs_next::cache_dir()
            .unwrap_or_else(|| dirs_next::home_dir().unwrap_or_default())
            .join(env!("CARGO_PKG_NAME")),
    )
    .unwrap_or_else(|_| panic!("Unable to create the cache directory for {}", env!("CARGO_PKG_NAME")));

    dirs_next::cache_dir()
        .unwrap_or_else(|| dirs_next::home_dir().unwrap_or_default())
        .join(env!("CARGO_PKG_NAME"))
        .join(format!("{}-{}.cache", datatype, hash))
        .display()
        .to_string()
}
