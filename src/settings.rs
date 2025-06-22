use clap::ValueEnum;
use cli_settings_derive::cli_settings;
use serde::Deserialize;

#[derive(ValueEnum, Clone, Debug, Deserialize, Default)]
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

#[derive(Clone, Debug, Deserialize, Default)]
pub struct WaybarConfig {
    pub text: String,
    pub tooltip: String,
    pub hot_temperature: i32,
    pub cold_temperature: i32,
}

#[derive(Debug, Clone)]
#[cli_settings]
#[cli_settings_file = "#[serde_with::serde_as]#[derive(serde::Deserialize)]"]
#[cli_settings_clap = "#[derive(clap::Parser)]#[command(name = \"outside\", version)]"]
pub struct Settings {
    // TODO: Break config file settings out into sections for each output format
    #[cli_settings_file]
    #[cli_settings_clap = "#[arg(short, long, help = \"'City, CA' or leave blank to auto-detect\")]"]
    pub location: String,

    #[cli_settings_file]
    #[cli_settings_clap = "#[arg(short, long, help = \"Units of measurement\")]"]
    pub units: Units,

    #[cli_settings_clap = "#[arg(short, long, help = \"Desired output format\")]"]
    pub output_format: OutputFormat,

    #[cli_settings_clap = "#[arg(long, help = \"Don't use cached location and weather data\")]"]
    #[cli_settings_default = "true"]
    pub use_cache: bool,

    #[cli_settings_file]
    pub waybar: WaybarConfig,
}
