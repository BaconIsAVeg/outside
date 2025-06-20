use crate::api::{LocationData, weather};
use crate::utils::units;

pub mod api;
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
}
