pub mod api;
pub mod context;
pub mod output;
pub mod settings;
pub mod utils;

use crate::api::location::LocationData;
use crate::api::weather;
use crate::settings::{Settings, Units};
use anyhow::Result;

/// Main entry point for the outside weather CLI application.
///
/// This function orchestrates the complete weather data pipeline:
/// 1. Builds configuration from config file and CLI arguments
/// 2. Resolves location data (with caching)
/// 3. Fetches weather data from Open-Meteo API (with caching)
/// 4. Builds context for template rendering
/// 5. Renders and outputs the weather information in the specified format
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if any step fails.
fn main() -> Result<()> {
    let config_file = dirs_next::config_dir()
        .unwrap_or_else(|| dirs_next::home_dir().unwrap_or_default())
        .join(env!("CARGO_PKG_NAME"))
        .join("config.yaml");

    let s = Settings::build(vec![config_file], std::env::args_os())?;

    let loc = LocationData::get_cached(s.clone())?;
    let weather = weather::Weather::get_cached(loc.latitude, loc.longitude, s.clone())?;

    let context = context::Context::build(weather, loc);

    let output = s.output.render_fn()(context, s);

    println!("{}", output);
    Ok(())
}
