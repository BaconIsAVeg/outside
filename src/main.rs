pub mod api;
pub mod context;
pub mod output;
pub mod settings;
pub mod utils;

use crate::api::location::LocationData;
use crate::api::weather;
use crate::output::*;
use crate::settings::{OutputFormat, Settings, Units};

fn main() {
    let config_file = dirs_next::config_dir()
        .unwrap_or_else(|| dirs_next::home_dir().unwrap_or_default())
        .join(env!("CARGO_PKG_NAME"))
        .join("config.yaml");

    let s = Settings::build(vec![config_file], std::env::args_os()).unwrap();

    let loc = LocationData::get_cached(s.clone());
    let weather = weather::Weather::get_cached(loc.latitude, loc.longitude, s.clone());

    let context = context::Context::build(weather, loc);

    let output = match s.output {
        OutputFormat::Simple => render_output::<simple::SimpleOutput>(context, s.clone()),
        OutputFormat::Waybar => render_output::<waybar::WaybarOutput>(context, s.clone()),
        OutputFormat::Json => render_output::<json::JsonOutput>(context, s.clone()),
        OutputFormat::Detailed => render_output::<detailed::DetailedOutput>(context, s.clone()),
    };

    println!("{}", output);
}
