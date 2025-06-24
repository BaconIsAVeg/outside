use crate::utils::*;
use crate::Settings;
use crate::Units;

use isahc::prelude::*;
use savefile::prelude::*;
use savefile_derive::Savefile;
use serde::{Deserialize, Serialize};
use url::Url;

macro_rules! string_vec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[derive(Default, Serialize, Deserialize, Debug, Savefile)]
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

impl Weather {
    pub fn get_cached(lat: f64, lon: f64, s: Settings) -> Self {
        let filename = get_cached_file("weather", &s.location, s.units.to_owned());
        if cfg!(debug_assertions) {
            println!("Weather cache file: {}", filename);
        }
        let now = get_now();

        let unit_strings = match s.units.to_owned() {
            Units::Metric => unitstrings::UnitStrings::metric(),
            Units::Imperial => unitstrings::UnitStrings::imperial(),
        };

        let wd: Weather = load_file(&filename, 0).unwrap_or_default();

        if wd.latitude == lat && wd.longitude == lon && wd.created_at > 0 && now - wd.created_at < 580 {
            if cfg!(debug_assertions) {
                println!("Using cached weather data");
            }
            return wd;
        }

        let mut data = Self::fetch(lat, lon, unit_strings);
        data.latitude = format!("{:.1}", data.latitude).parse().unwrap_or(0.0);
        data.longitude = format!("{:.1}", data.longitude).parse().unwrap_or(0.0);
        data.created_at = now;

        match save_file(&filename, 0, &data) {
            Ok(_) => {
                if cfg!(debug_assertions) {
                    println!("Wrote weather data to disk");
                }
            },
            Err(e) => eprintln!("Unable to save weather data to disk: {:#?}", e),
        }

        data
    }

    fn fetch(lat: f64, lon: f64, units: unitstrings::UnitStrings) -> Self {
        let api_url = build_url(
            lat.to_string().as_str(),
            lon.to_string().as_str(),
            units.temperature.as_str(),
            units.wind_speed.as_str(),
            units.precipitation.as_str(),
        );

        let mut response = isahc::get(api_url).expect("Unable to send Weather request");
        if !response.status().is_success() {
            panic!("Unable to fetch weather: {}", response.status());
        }
        let body = response.text().expect("Unable to read Weather response body");
        serde_json::from_str(&body).expect("Unable to parse Weather JSON response")
    }
}

fn build_url(
    lat: &str,
    lon: &str,
    temperature_unit: &str,
    wind_speed_unit: &str,
    precipitation_unit: &str,
) -> String {
    let base_url = "https://api.open-meteo.com/v1/forecast";
    let mut url = Url::parse(base_url).expect("Unable to parse base URL");

    let current_fields = string_vec![
        "temperature_2m",
        "relative_humidity_2m",
        "apparent_temperature",
        "wind_speed_10m",
        "wind_direction_10m",
        "wind_gusts_10m",
        "precipitation",
        "weather_code",
        "pressure_msl"
    ];

    let daily_fields = string_vec![
        "sunrise",
        "sunset",
        "weather_code",
        "temperature_2m_max",
        "temperature_2m_min",
        "precipitation_sum",
        "precipitation_hours",
        "precipitation_probability_max",
        "uv_index_max"
    ];

    url.query_pairs_mut()
        .append_pair("latitude", lat)
        .append_pair("longitude", lon)
        .append_pair("timezone", "auto")
        .append_pair("forecast_days", "7")
        .append_pair("current", &current_fields.join(","))
        .append_pair("daily", &daily_fields.join(","))
        .append_pair("temperature_unit", temperature_unit)
        .append_pair("wind_speed_unit", wind_speed_unit)
        .append_pair("precipitation_unit", precipitation_unit);

    if cfg!(debug_assertions) {
        println!("Weather API: {:#?}", url);
    }
    url.to_string()
}
