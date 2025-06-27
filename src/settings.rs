use clap::ValueEnum;
use cli_settings_derive::cli_settings;
use serde::{Deserialize, Serialize};

#[derive(ValueEnum, Clone, Debug, Serialize, Deserialize, Default)]
pub enum Units {
    #[default]
    Metric = 0,
    Imperial = 1,
}

#[derive(ValueEnum, Clone, Debug, Deserialize, Default)]
pub enum OutputFormat {
    #[default]
    Simple = 0,
    Detailed = 1,
    Json = 2,
    Waybar = 3,
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
