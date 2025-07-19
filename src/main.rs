pub mod api;
pub mod context;
pub mod output;
pub mod settings;
pub mod tui;
pub mod utils;

use crate::api::location::LocationData;
use crate::api::weather;
use crate::settings::{OutputFormat, Settings, Units};
use anyhow::Result;
use std::time::Duration;
use tokio::signal;
use tokio::time::interval;

/// Main entry point for the outside weather CLI application.
///
/// This function orchestrates the complete weather data pipeline:
/// 1. Builds configuration from config file and CLI arguments
/// 2. Resolves location data (with caching)
/// 3. Fetches weather data from Open-Meteo API (with caching)
/// 4. Builds context for template rendering
/// 5. Renders and outputs the weather information in the specified format
///
/// Supports both single-run mode and streaming mode for continuous output.
/// In streaming mode, weather data is fetched and output at regular intervals
/// until the program receives a termination signal.
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if any step fails.
#[tokio::main]
async fn main() -> Result<()> {
    let config_file = dirs_next::config_dir()
        .unwrap_or_else(|| dirs_next::home_dir().unwrap_or_default())
        .join(env!("CARGO_PKG_NAME"))
        .join("config.yaml");

    let s = Settings::build(vec![config_file], std::env::args_os())?;

    // TUI mode is incompatible with streaming mode
    if s.stream && matches!(s.output, OutputFormat::Tui) {
        eprintln!("Error: TUI mode cannot be used with streaming mode.");
        std::process::exit(1);
    }

    if s.stream {
        run_streaming_mode(s).await
    } else {
        run_single_mode(s).await
    }
}

/// Runs the application in streaming mode for continuous output.
///
/// Outputs weather data at regular intervals until interrupted by a signal.
/// This mode is particularly useful for status bars like Waybar that expect
/// continuous JSON output from external commands.
///
/// # Arguments
///
/// * `settings` - Application configuration including interval and output format
///
/// # Returns
///
/// Returns `Ok(())` when gracefully shutdown, or an error if critical failure occurs.
async fn run_streaming_mode(settings: Settings) -> Result<()> {
    let mut timer = interval(Duration::from_secs(settings.interval));

    // Output immediately on startup
    if let Err(e) = output_weather_data(&settings).await {
        eprintln!("Error fetching initial weather data: {}", e);
    }

    // Skip the first tick since interval.tick() fires immediately
    timer.tick().await;

    loop {
        tokio::select! {
            _ = timer.tick() => {
                if let Err(e) = output_weather_data(&settings).await {
                    eprintln!("Error fetching weather data: {}", e);
                    // Continue running even if one fetch fails
                    continue;
                }
            }
            _ = signal::ctrl_c() => {
                if cfg!(debug_assertions) {
                    eprintln!("Received interrupt signal, shutting down gracefully");
                }
                break;
            }
        }
    }

    Ok(())
}

/// Runs the application in single-run mode.
///
/// Fetches weather data once, outputs it, and exits. This is the traditional
/// behavior of the application.
///
/// # Arguments
///
/// * `settings` - Application configuration
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if any step fails.
async fn run_single_mode(settings: Settings) -> Result<()> {
    output_weather_data(&settings).await
}

/// Fetches weather data and outputs it according to the configured format.
///
/// This function encapsulates the core weather data pipeline that can be used
/// in both single-run and streaming modes.
///
/// # Arguments
///
/// * `settings` - Application configuration
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if fetching or output fails.
async fn output_weather_data(settings: &Settings) -> Result<()> {
    let loc = LocationData::get_cached(settings.clone())?;
    let weather = weather::Weather::get_cached(loc.latitude, loc.longitude, settings.clone())?;

    let context = context::Context::build(weather, loc, settings.clone());
    let output = settings.output.render_fn()(context, settings.clone());

    println!("{}", output);
    Ok(())
}
