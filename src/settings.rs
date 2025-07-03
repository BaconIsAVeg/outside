use crate::context::Context;
use crate::output::*;
use crate::utils::unitstrings::UnitStrings;
use crate::Settings as OutsideSettings;

use clap::ValueEnum;
use cli_settings_derive::cli_settings;
use serde::{Deserialize, Serialize};

#[derive(ValueEnum, Clone, Debug, Serialize, Deserialize, Default)]
pub enum Units {
    #[default]
    Metric,
    Imperial,
}

impl Units {
    /// Returns the string representation of the units for use in API calls.
    ///
    /// # Returns
    ///
    /// Returns "metric" for metric units or "imperial" for imperial units.
    pub fn as_str(&self) -> &'static str {
        match self {
            Units::Metric => "metric",
            Units::Imperial => "imperial",
        }
    }

    /// Converts the units enum to a `UnitStrings` struct for template rendering.
    ///
    /// # Returns
    ///
    /// Returns a `UnitStrings` struct containing the appropriate unit suffixes
    /// for display in output templates.
    pub fn to_unit_strings(&self) -> UnitStrings {
        match self {
            Units::Metric => UnitStrings::metric(),
            Units::Imperial => UnitStrings::imperial(),
        }
    }
}

#[derive(ValueEnum, Clone, Debug, Serialize, Deserialize, Default)]
pub enum OutputFormat {
    #[default]
    Simple,
    Detailed,
    Json,
    Waybar,
}

impl OutputFormat {
    /// Returns the appropriate rendering function for the selected output format.
    ///
    /// Each output format has its own implementation of the `Output` trait,
    /// and this method returns the correct rendering function to use.
    ///
    /// # Returns
    ///
    /// Returns a function pointer that takes a `Context` and `OutsideSettings`
    /// and returns a formatted string for the selected output format.
    pub fn render_fn(&self) -> fn(Context, OutsideSettings) -> String {
        match self {
            OutputFormat::Simple => render_output::<simple::SimpleOutput>,
            OutputFormat::Detailed => render_output::<detailed::DetailedOutput>,
            OutputFormat::Json => render_output::<json::JsonOutput>,
            OutputFormat::Waybar => render_output::<waybar::WaybarOutput>,
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Default)]
pub struct WaybarConfig {
    pub text: Option<String>,
    pub tooltip: Option<String>,
    pub hot_temperature: Option<f64>,
    pub cold_temperature: Option<f64>,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Default)]
pub struct SimpleConfig {
    pub template: Option<String>,
}

#[derive(Debug, Clone)]
#[cli_settings]
#[cli_settings_file = "#[serde_with::serde_as]#[derive(serde::Deserialize)]"]
/// A multi-purpose CLI weather client that uses the Open-Meteo API.
#[cli_settings_clap = "#[derive(clap::Parser)]#[command(name = \"outside\", version, verbatim_doc_comment)]"]
pub struct Settings {
    /// The location for which to fetch the weather data:
    ///     Must be in the format 'City, Country' i.e. New York, US
    ///     Or leave blank to auto-detect using your IP address
    #[cli_settings_file]
    #[cli_settings_clap = "#[arg(short, long, verbatim_doc_comment)]"]
    pub location: String,

    /// The units of measurement for the weather data
    ///   :
    #[cli_settings_file]
    #[cli_settings_clap = "#[arg(short, long, verbatim_doc_comment)]"]
    pub units: Units,

    /// Which of the avilable output formats to use:
    ///     simple: A simple text output, suitable for Polybar/Lemonbar/etc
    ///     detailed: A detailed text output that includes a 7 day forecast
    ///     json: A JSON output that includes all context data
    ///     waybar: A JSON output formatted for Waybar
    ///   :
    #[cli_settings_clap = "#[arg(short, long, verbatim_doc_comment)]"]
    pub output: OutputFormat,

    #[cli_settings_file]
    pub simple: SimpleConfig,

    #[cli_settings_file]
    pub waybar: WaybarConfig,
}
