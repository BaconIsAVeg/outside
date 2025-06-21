use crate::utils::units::Units;
use disk::*;
use isahc::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;

json!(Weather, Dir::Data, env!("CARGO_PKG_NAME"), "weather", "data");
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Weather {
    pub current: Current,
    pub current_units: CurrentUnits,
    pub elevation: f64,
    pub timezone: String,
    pub utc_offset_seconds: i32,
    pub daily: Daily,
    pub daily_units: DailyUnits,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(default)]
    pub created_at: u64,
}

#[derive(Default, Serialize, Deserialize, Debug)]
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

#[derive(Default, Serialize, Deserialize, Debug)]
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

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Daily {
    pub time: Vec<String>,
    pub sunrise: Vec<String>,
    pub sunset: Vec<String>,
    pub uv_index_max: Vec<f64>,
    pub precipitation_sum: Vec<f64>,
    pub precipitation_hours: Vec<f64>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct DailyUnits {
    pub sunrise: String,
    pub sunset: String,
    pub uv_index_max: String,
    pub precipitation_sum: String,
    pub precipitation_hours: String,
}

impl Weather {
    pub fn get_cached(lat: f64, lon: f64, units: Units) -> Self {
        let now: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let wd = Weather::from_file().unwrap_or_default();

        if wd.latitude == lat
            && wd.longitude == lon
            && wd.created_at > 0
            && now - wd.created_at < 600
        {
            if cfg!(debug_assertions) {
                println!("Using cached weather data: {:#?}", wd);
            }
            return wd;
        }

        let mut data = Self::fetch(lat, lon, units);
        data.latitude = format!("{:.1}", data.latitude).parse().unwrap_or(0.0);
        data.longitude = format!("{:.1}", data.longitude).parse().unwrap_or(0.0);
        data.created_at = now;

        match data.save_atomic() {
            Ok(_) => {
                if cfg!(debug_assertions) {
                    println!("Wrote weather data to disk: {:#?}", data);
                }
            },
            Err(e) => eprintln!("Failed saving weather data to disk: {:#?}", e),
        }

        data
    }

    pub fn fetch(lat: f64, lon: f64, units: Units) -> Self {
        let api_url = Self::build_url(
            lat,
            lon,
            units.temperature.as_str(),
            units.wind_speed.as_str(),
            units.precipitation.as_str(),
        );

        let mut response = isahc::get(api_url).expect("Failed to send Weather request");
        if !response.status().is_success() {
            panic!("Failed to fetch weather: {}", response.status());
        }
        let body = response.text().expect("Failed to read Weather response body");
        serde_json::from_str(&body).expect("Failed to parse Weather JSON response")
    }

    pub fn build_url(
        lat: f64,
        lon: f64,
        temperature_unit: &str,
        wind_speed_unit: &str,
        precipitation_unit: &str,
    ) -> String {
        let base_url = "https://api.open-meteo.com/v1/forecast";
        let mut url = Url::parse(base_url).expect("Failed to parse base URL");

        url.query_pairs_mut()
        .append_pair("latitude", &lat.to_string())
        .append_pair("longitude", &lon.to_string())
        .append_pair("timezone", "auto")
        .append_pair("current", "temperature_2m,relative_humidity_2m,apparent_temperature,wind_speed_10m,wind_direction_10m,wind_gusts_10m,precipitation,weather_code,pressure_msl")
        .append_pair("daily", "sunrise,sunset,uv_index_max,precipitation_sum,precipitation_hours")
        .append_pair("temperature_unit", temperature_unit)
        .append_pair("wind_speed_unit", wind_speed_unit)
        .append_pair("precipitation_unit", precipitation_unit);

        if cfg!(debug_assertions) {
            println!("Weather API: {:#?}", url);
        }
        url.to_string()
    }
}
