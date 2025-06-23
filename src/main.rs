use crate::api::{weather, LocationData};
use crate::output::*;
use crate::settings::{OutputFormat, Settings, Units};

pub mod api;
pub mod context;
pub mod output;
pub mod settings;
pub mod utils;

fn main() {
    let dirs = xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME"));
    dirs.create_cache_directory("").unwrap_or_else(|e| {
        eprintln!("Unable to create cache directory: {}", e);
        std::process::exit(1);
    });

    let s = Settings::build(dirs.find_config_files("config.yaml"), std::env::args_os()).unwrap();

    let loc = LocationData::get_cached(s.clone());
    let weather = weather::Weather::get_cached(loc.latitude, loc.longitude, s.clone());

    let context = context::Context::build(weather, loc);
    if cfg!(debug_assertions) {
        println!("Context: {:#?}", context);
        println!("Settings: {:#?}", s);
    }

    let output = match s.output_format {
        OutputFormat::Simple => render_output::<simple::SimpleOutput>(context, s.clone()),
        OutputFormat::Waybar => render_output::<waybar::WaybarOutput>(context, s.clone()),
        OutputFormat::Json => render_output::<json::JsonOutput>(context, s.clone()),
        OutputFormat::Detailed => render_output::<detailed::DetailedOutput>(context, s.clone()),
    };

    println!("{}", output);
}
