use crate::api::LocationData;
use crate::output::Output;
use crate::utils::lookups;
use crate::weather::Weather;

#[derive(serde::Deserialize, Debug)]
pub struct PlainOutput {
    pub template: String,
}

impl Output for PlainOutput {
    fn new(weather: Weather, _location: LocationData) -> Self {
        let current = &weather.current;
        let units = &weather.current_units;
        let template = format!(
            "{} {}{} 󰖝 {}{}",
            lookups::weather_code_to_icon(current.weather_code),
            current.temperature_2m,
            units.temperature_2m,
            current.wind_speed_10m,
            units.wind_speed_10m,
        );
        PlainOutput { template }
    }

    fn render(&self) -> String {
        self.template.clone()
    }
}
