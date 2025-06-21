use crate::api::{LocationData, weather};
use crate::utils::units;

pub mod api;
pub mod context;
pub mod output;
pub mod settings;
pub mod utils;

fn main() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME"));
    let configuration =
        settings::Settings::build(xdg_dirs.find_config_files("config.yaml"), std::env::args_os())
            .unwrap();

    let units = match configuration.units {
        settings::Units::Metric => units::Units::metric(),
        settings::Units::Imperial => units::Units::imperial(),
    };

    let location: LocationData = LocationData::lookup(configuration.location.to_owned());

    let weather = weather::Weather::fetch(location.latitude, location.longitude, units);

    if cfg!(debug_assertions) {
        println!("{:#?}", configuration);
        println!("{:#?}", location);

        println!("{:#?}", weather.current);
        println!("{:#?}", weather.daily);
    }

    let context = context::Context::build(weather, location);

    match configuration.output_format {
        settings::OutputFormat::Simple => {
            let output = output::render_output::<output::simple::SimpleOutput>(context);
            println!("{}", output);
        },
        settings::OutputFormat::Waybar => {
            let output = output::render_output::<output::waybar::WaybarOutput>(context);
            println!("{}", output);
        },
        settings::OutputFormat::Json => {
            let output = output::render_output::<output::json::JsonOutput>(context);
            println!("{}", output);
        },
        settings::OutputFormat::Detailed => {
            let output = output::render_output::<output::detailed::DetailedOutput>(context);
            println!("{}", output);
        },
    }
}
