use crate::api::client;
use crate::utils;
use crate::Settings;

use anyhow::{Context, Result};
use savefile::prelude::*;
use savefile_derive::Savefile;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, Savefile)]
pub struct Weather {
    pub current: Current,
    pub current_units: CurrentUnits,
    pub elevation: f64,
    pub timezone: String,
    pub utc_offset_seconds: i32,
    pub daily: Daily,
    pub daily_units: DailyUnits,
    pub hourly: Hourly,
    pub hourly_units: HourlyUnits,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(default)]
    pub created_at: u64,
}

#[derive(Default, Serialize, Deserialize, Debug, Savefile)]
pub struct Current {
    pub apparent_temperature: f64,
    pub interval: i32,
    pub precipitation: f64,
    pub pressure_msl: f64,
    pub relative_humidity_2m: i32,
    pub temperature_2m: f64,
    pub weather_code: i32,
    pub wind_direction_10m: i32,
    pub wind_speed_10m: f64,
    pub wind_gusts_10m: f64,
}

#[derive(Default, Serialize, Deserialize, Debug, Savefile)]
pub struct CurrentUnits {
    pub apparent_temperature: String,
    pub interval: String,
    pub precipitation: String,
    pub pressure_msl: String,
    pub relative_humidity_2m: String,
    pub temperature_2m: String,
    pub weather_code: String,
    pub wind_direction_10m: String,
    pub wind_speed_10m: String,
    pub wind_gusts_10m: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Savefile)]
pub struct Daily {
    pub time: Vec<String>,
    pub weather_code: Vec<i32>,
    pub sunrise: Vec<String>,
    pub sunset: Vec<String>,
    pub uv_index_max: Vec<f64>,
    pub precipitation_sum: Vec<f64>,
    pub precipitation_hours: Vec<f64>,
    pub precipitation_probability_max: Vec<i32>,
    pub temperature_2m_max: Vec<f64>,
    pub temperature_2m_min: Vec<f64>,
}

#[derive(Default, Serialize, Deserialize, Debug, Savefile)]
pub struct DailyUnits {
    pub time: String,
    pub weather_code: String,
    pub sunrise: String,
    pub sunset: String,
    pub uv_index_max: String,
    pub precipitation_sum: String,
    pub precipitation_hours: String,
    pub precipitation_probability_max: String,
    pub temperature_2m_max: String,
    pub temperature_2m_min: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Savefile)]
pub struct Hourly {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f64>,
    pub precipitation_probability: Vec<i32>,
    pub precipitation: Vec<f64>,
    pub weather_code: Vec<i32>,
}

#[derive(Default, Serialize, Deserialize, Debug, Savefile)]
pub struct HourlyUnits {
    pub time: String,
    pub temperature_2m: String,
    pub precipitation_probability: String,
    pub precipitation: String,
    pub weather_code: String,
}

impl Weather {
    /// Retrieves weather data for the specified coordinates, using cached data if available.
    ///
    /// Weather data is cached for 10 minutes (580 seconds) to reduce API calls.
    /// If cached data is found for the same coordinates and is still fresh, it will be returned.
    /// Otherwise, fresh data will be fetched from the Open-Meteo API.
    ///
    /// # Arguments
    ///
    /// * `lat` - Latitude coordinate for the weather location
    /// * `lon` - Longitude coordinate for the weather location
    /// * `s` - Settings containing units and location information for caching
    ///
    /// # Returns
    ///
    /// Returns weather data on success, or an error if fetching fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The API request fails
    /// - The response cannot be parsed as JSON
    /// - Network connectivity issues occur
    pub fn get_cached(lat: f64, lon: f64, s: Settings) -> Result<Self> {
        let filename = utils::cache::get_cached_file("weather", &s.location);
        let now = utils::get_now();

        let metric_unit_strings = utils::unitstrings::UnitStrings::metric();

        let wd: Weather = load_file(&filename, 0).unwrap_or_default();

        if wd.latitude == lat && wd.longitude == lon && wd.created_at > 0 && now - wd.created_at < 600 {
            return Ok(wd);
        }

        let mut data =
            Self::fetch(lat, lon, metric_unit_strings).with_context(|| "Failed to fetch weather data")?;
        data.latitude = format!("{:.1}", data.latitude).parse().unwrap_or(0.0);
        data.longitude = format!("{:.1}", data.longitude).parse().unwrap_or(0.0);
        data.created_at = now;

        match save_file(&filename, 0, &data) {
            Ok(_) => {},
            Err(e) => eprintln!("Unable to save weather data to disk: {e:#?}"),
        }

        Ok(data)
    }

    /// Fetches fresh weather data from the Open-Meteo API.
    ///
    /// Constructs the API URL with the appropriate parameters for current weather,
    /// 7-day forecast, and unit preferences, then makes the HTTP request.
    ///
    /// # Arguments
    ///
    /// * `lat` - Latitude coordinate for the weather location
    /// * `lon` - Longitude coordinate for the weather location
    /// * `units` - Unit system for temperature, wind speed, and precipitation
    ///
    /// # Returns
    ///
    /// Returns parsed weather data on success, or an error if the request fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The HTTP request fails
    /// - The JSON response cannot be parsed
    /// - The API returns an error response
    fn fetch(lat: f64, lon: f64, units: utils::unitstrings::UnitStrings) -> Result<Self> {
        let base_url = "https://api.open-meteo.com/v1/forecast";

        // https://api.open-meteo.com/v1/forecast\?latitude\=51.30011\&longitude\=-114.03528\&daily\=weather_code,temperature_2m_max,temperature_2m_min,sunset,sunrise,precipitation_hours,precipitation_probability_max\&hourly\=temperature_2m,precipitation_probability,precipitation\&current\=temperature_2m,apparent_temperature,wind_speed_10m,wind_direction_10m,wind_gusts_10m,precipitation,weather_code,pressure_msl,relative_humidity_2m\&timezone\=America%2FDenver
        let hourly_fields =
            ["temperature_2m", "precipitation_probability", "precipitation", "weather_code"].join(",");

        let current_fields = [
            "temperature_2m",
            "relative_humidity_2m",
            "apparent_temperature",
            "wind_speed_10m",
            "wind_direction_10m",
            "wind_gusts_10m",
            "precipitation",
            "weather_code",
            "pressure_msl",
        ]
        .join(",");

        let daily_fields = [
            "sunrise",
            "sunset",
            "weather_code",
            "temperature_2m_max",
            "temperature_2m_min",
            "precipitation_sum",
            "precipitation_hours",
            "precipitation_probability_max",
            "uv_index_max",
        ]
        .join(",");

        let lat_str = lat.to_string();
        let lon_str = lon.to_string();

        let params: Vec<(&str, &str)> = vec![
            ("latitude", lat_str.as_str()),
            ("longitude", lon_str.as_str()),
            ("timezone", "auto"),
            ("forecast_days", "7"),
            ("current", current_fields.as_str()),
            ("daily", daily_fields.as_str()),
            ("hourly", hourly_fields.as_str()),
            ("temperature_unit", units.temperature.as_str()),
            ("wind_speed_unit", units.wind_speed.as_str()),
            ("precipitation_unit", units.precipitation.as_str()),
        ];

        let api_url = utils::urls::builder(base_url, params);

        let body = client::get_with_retry(&api_url, 2)
            .with_context(|| "Unable to fetch weather data from the Open-Meteo API endpoint")?;

        serde_json::from_str(&body).with_context(|| "Unable to parse weather response JSON")
    }
}
