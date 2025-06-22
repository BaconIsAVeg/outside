use crate::api::{weather, LocationData};
use crate::output::*;
use crate::settings::{OutputFormat, Settings, Units};
use crate::utils::units;
use xdg::BaseDirectories;

pub mod api;
pub mod context;
pub mod output;
pub mod settings;
pub mod utils;

fn main() {
    let dirs = BaseDirectories::with_prefix(env!("CARGO_PKG_NAME"));
    let s = Settings::build(dirs.find_config_files("config.yaml"), std::env::args_os()).unwrap();
    let units = match s.units {
        Units::Metric => units::Units::metric(),
        Units::Imperial => units::Units::imperial(),
    };

    let loc = LocationData::get_cached(s.location.to_owned(), s.use_cache);
    let weather = weather::Weather::get_cached(loc.latitude, loc.longitude, units, s.use_cache);

    let context = context::Context::build(weather, loc);
    let output = match s.output_format {
        OutputFormat::Simple => render_output::<simple::SimpleOutput>(context),
        OutputFormat::Waybar => render_output::<waybar::WaybarOutput>(context),
        OutputFormat::Json => render_output::<json::JsonOutput>(context),
        OutputFormat::Detailed => render_output::<detailed::DetailedOutput>(context),
    };
    println!("{}", output);
}
