use crate::api::LocationData;
use crate::utils::conversions;
use crate::utils::mappings;
use crate::utils::*;
use crate::weather::Weather;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Context {
    pub city: String,
    pub country: String,
    pub temperature: f64,
    pub temperature_round: String,
    pub feels_like: f64,
    pub feels_like_round: String,
    pub temperature_unit: String,
    pub wind_speed: f64,
    pub wind_speed_round: String,
    pub wind_gusts: f64,
    pub wind_gusts_round: String,
    pub wind_speed_unit: String,
    pub wind_direction: i32,
    pub wind_compass: String,
    pub weather_code: i32,
    pub weather_icon: String,
    pub weather_description: String,
    pub openweather_code: String,
    pub humidity: i32,
    pub humidity_unit: String,
    pub pressure: f64,
    pub pressure_round: String,
    pub pressure_unit: String,
    pub sunrise: String,
    pub sunset: String,
    pub uv_index: f64,
    pub precipitation_chance: i32,
    pub precipitation_sum: f64,
    pub precipitation_unit: String,
    pub precipitation_hours: f64,
    pub cache_age: u64,
}

impl Context {
    pub fn build(weather: Weather, location: LocationData) -> Self {
        // TODO: Figure out how to make this global
        let now = get_now();

        let current = &weather.current;
        let daily = &weather.daily;
        let daily_units = &weather.daily_units;
        let units = &weather.current_units;
        let openweather_code = mappings::meteo2openweather_codes(current.weather_code);
        let wind_compass = mappings::degrees2compass(current.wind_direction_10m as f64);

        let weather_description = mappings::weather_description(current.weather_code);
        let weather_icon = mappings::weather_code2icon(current.weather_code);

        let sunrise = conversions::iso8601_to_time(daily.sunrise[0].clone());
        let sunset = conversions::iso8601_to_time(daily.sunset[0].clone());

        let cache_age = now - weather.created_at;

        Context {
            city: location.city,
            country: location.country_code,
            temperature: current.temperature_2m,
            feels_like: current.apparent_temperature,
            temperature_unit: units.temperature_2m.clone(),
            wind_speed: current.wind_speed_10m,
            wind_gusts: current.wind_gusts_10m,
            wind_speed_unit: units.wind_speed_10m.clone(),
            wind_direction: current.wind_direction_10m,
            wind_compass,
            weather_code: current.weather_code,
            weather_icon,
            weather_description,
            openweather_code,
            humidity: current.relative_humidity_2m,
            humidity_unit: units.relative_humidity_2m.clone(),
            pressure: current.pressure_msl,
            pressure_unit: units.pressure_msl.clone(),
            sunrise,
            sunset,
            uv_index: daily.uv_index_max[0],
            precipitation_chance: daily.precipitation_probability_max[0],
            precipitation_sum: daily.precipitation_sum[0],
            precipitation_unit: daily_units.precipitation_sum.clone(),
            precipitation_hours: daily.precipitation_hours[0],

            temperature_round: format!("{:.0}", current.temperature_2m.round()),
            feels_like_round: format!("{:.0}", current.apparent_temperature.round()),
            wind_speed_round: format!("{:.0}", current.wind_speed_10m.round()),
            wind_gusts_round: format!("{:.0}", current.wind_gusts_10m.round()),
            pressure_round: format!("{:.0}", current.pressure_msl.round()),

            cache_age,
        }
    }
}
