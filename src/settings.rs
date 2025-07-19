use crate::context::Context;
use crate::output::*;
use crate::utils::unitstrings::UnitStrings;
use crate::Settings as OutsideSettings;

use clap::ValueEnum;
use cli_settings_derive::cli_settings;
use serde::{Deserialize, Serialize};

#[derive(ValueEnum, Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
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
    Tui,
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
            OutputFormat::Tui => render_output::<crate::tui::TuiOutput>,
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
/// A multi-purpose weather client for your terminal
#[cli_settings_clap = "#[derive(clap::Parser)]#[command(name = \"outside\", version, verbatim_doc_comment)]"]
pub struct Settings {
    /// Location to fetch weather data for,
    /// leave blank to auto-detect using your IP address
    #[cli_settings_file]
    #[cli_settings_clap = "#[arg(short, long, verbatim_doc_comment)]"]
    pub location: String,

    /// Units of measurement
    #[cli_settings_file]
    #[cli_settings_clap = "#[arg(short, long, verbatim_doc_comment)]"]
    pub units: Units,

    /// Display format
    #[cli_settings_clap = "#[arg(short, long, verbatim_doc_comment)]"]
    pub output: OutputFormat,

    /// Enable streaming mode for continuous output
    #[cli_settings_clap = "#[arg(short, long, action = clap::ArgAction::SetTrue, verbatim_doc_comment)]"]
    pub stream: bool,

    /// Interval in seconds between streaming updates
    #[cli_settings_file]
    #[cli_settings_clap = "#[arg(short, long, default_value = \"30\", verbatim_doc_comment)]"]
    pub interval: u64,

    #[cli_settings_file]
    pub simple: SimpleConfig,

    #[cli_settings_file]
    pub waybar: WaybarConfig,
}
