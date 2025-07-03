use crate::utils::conversions;
use crate::utils::mappings;
use crate::utils::*;
use crate::weather::Weather;
use crate::LocationData;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Context {
    pub city: String,
    pub country: String,
    pub temperature: f64,
    pub temperature_low: f64,
    pub temperature_high: f64,
    pub feels_like: f64,
    pub temperature_unit: String,
    pub wind_speed: f64,
    pub wind_gusts: f64,
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
    pub pressure_unit: String,
    pub sunrise: String,
    pub sunset: String,
    pub uv_index: f64,
    pub precipitation_chance: i32,
    pub precipitation_sum: f64,
    pub precipitation_unit: String,
    pub precipitation_hours: f64,
    pub forecast: Vec<ContextDaily>,
    pub cache_age: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ContextDaily {
    pub date: String,
    pub weather_code: i32,
    pub weather_icon: String,
    pub weather_description: String,
    pub openweather_code: String,
    pub uv_index: f64,
    pub precipitation_sum: f64,
    pub precipitation_hours: f64,
    pub precipitation_chance: i32,
    pub temperature_high: f64,
    pub temperature_low: f64,
}

impl Context {
    /// Builds a unified context structure from weather data and location information.
    ///
    /// This function transforms raw API data into a structured format suitable for
    /// template rendering. It combines current weather conditions, daily forecasts,
    /// and location data into a single context object with processed values.
    ///
    /// The function performs several data transformations:
    /// - Converts weather codes to human-readable descriptions and icons
    /// - Transforms wind direction degrees to compass directions
    /// - Converts ISO8601 timestamps to human-readable time/date strings
    /// - Calculates cache age for freshness indication
    /// - Builds a 7-day forecast array with processed daily data
    ///
    /// # Arguments
    ///
    /// * `weather` - Weather data structure containing current conditions and forecasts
    /// * `location` - Location data containing city, country, and coordinates
    ///
    /// # Returns
    ///
    /// Returns a `Context` struct containing all processed weather and location data
    /// ready for template rendering across different output formats.
    pub fn build(weather: Weather, location: LocationData) -> Self {
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

        let dailies: Vec<ContextDaily> = daily
            .time
            .iter()
            .enumerate()
            .map(|(i, date)| ContextDaily {
                date: conversions::iso8601_to_date(date.clone()),
                weather_code: daily.weather_code[i],
                weather_icon: mappings::weather_code2icon(daily.weather_code[i]),
                weather_description: mappings::weather_description(daily.weather_code[i]),
                openweather_code: mappings::meteo2openweather_codes(daily.weather_code[i]),
                uv_index: daily.uv_index_max[i],
                precipitation_sum: daily.precipitation_sum[i],
                precipitation_hours: daily.precipitation_hours[i],
                precipitation_chance: daily.precipitation_probability_max[i],
                temperature_high: daily.temperature_2m_max[i],
                temperature_low: daily.temperature_2m_min[i],
            })
            .collect();

        Context {
            city: location.city,
            country: location.country_code,
            temperature: current.temperature_2m,
            temperature_low: daily.temperature_2m_min[0],
            temperature_high: daily.temperature_2m_max[0],
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
            forecast: dailies,

            cache_age,
        }
    }
}
