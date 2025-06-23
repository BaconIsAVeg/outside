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
#[cli_settings_clap = "#[derive(clap::Parser)]#[command(name = \"outside\", version)]"]
pub struct Settings {
    #[cli_settings_file]
    #[cli_settings_clap = "#[arg(short, long, help = \"'City, CA' or leave blank to auto-detect\")]"]
    pub location: String,

    #[cli_settings_file]
    #[cli_settings_clap = "#[arg(short, long, help = \"Units of measurement\")]"]
    pub units: Units,

    #[cli_settings_clap = "#[arg(short, long, help = \"Desired output format\")]"]
    pub output_format: OutputFormat,

    #[cli_settings_file]
    pub simple: SimpleConfig,

    #[cli_settings_file]
    pub waybar: WaybarConfig,
}
