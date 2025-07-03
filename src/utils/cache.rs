use crate::Units;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Generates a cache file path for the given data type, content, and units.
///
/// Creates a hashed filename to ensure unique cache files for different
/// locations and unit combinations. The cache directory is created if it
/// doesn't exist, and the filename includes a hash of the content and units
/// to prevent cache conflicts.
///
/// # Arguments
///
/// * `datatype` - The type of data being cached (e.g., "weather", "location")
/// * `content` - The content identifier (e.g., location string)
/// * `u` - The units system being used (affects cache key)
///
/// # Returns
///
/// Returns the full path to the cache file as a string.
///
/// # Panics
///
/// Panics if the cache directory cannot be created.
pub fn get_cached_file(datatype: &str, content: &str, u: Units) -> String {
    let mut hasher = DefaultHasher::new();
    let f = format!("{}-{}", content, u.as_str());
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
