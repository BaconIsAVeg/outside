use crate::utils::conversions;
use crate::utils::mappings;
use crate::utils::*;
use crate::weather::Weather;
use crate::{LocationData, Settings, Units};

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
    pub precipitation_start: Option<i32>,
    pub precipitation_end: Option<i32>,
    pub forecast: Vec<ContextDaily>,
    pub hourly: Vec<ContextHourly>,
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ContextHourly {
    pub time: String,
    pub temperature: f64,
    pub precipitation_probability: i32,
    pub precipitation: f64,
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
    /// * `weather` - Weather data structure containing current conditions and forecasts (always in metric)
    /// * `location` - Location data containing city, country, and coordinates
    /// * `settings` - Settings containing units and other configuration
    ///
    /// # Returns
    ///
    /// Returns a `Context` struct containing all processed weather and location data
    /// ready for template rendering across different output formats.
    pub fn build(weather: Weather, location: LocationData, settings: Settings) -> Self {
        let now = get_now();

        let current = &weather.current;
        let daily = &weather.daily;
        let hourly = &weather.hourly;
        let openweather_code = mappings::meteo2openweather_codes(current.weather_code);
        let wind_compass = mappings::degrees2compass(current.wind_direction_10m as f64);

        let weather_description = mappings::weather_description(current.weather_code);
        let weather_icon = mappings::weather_code2icon(current.weather_code);

        let sunrise = conversions::iso8601_to_time(daily.sunrise[0].clone());
        let sunset = conversions::iso8601_to_time(daily.sunset[0].clone());

        let cache_age = now - weather.created_at;

        // Convert values based on user settings
        let is_imperial = settings.units == Units::Imperial;
        
        // Convert current weather values
        let temperature = if is_imperial { conversions::celsius_to_fahrenheit(current.temperature_2m) } else { current.temperature_2m };
        let feels_like = if is_imperial { conversions::celsius_to_fahrenheit(current.apparent_temperature) } else { current.apparent_temperature };
        let wind_speed = if is_imperial { conversions::kmh_to_mph(current.wind_speed_10m) } else { current.wind_speed_10m };
        let wind_gusts = if is_imperial { conversions::kmh_to_mph(current.wind_gusts_10m) } else { current.wind_gusts_10m };
        
        // Convert daily values for today
        let temperature_low = if is_imperial { conversions::celsius_to_fahrenheit(daily.temperature_2m_min[0]) } else { daily.temperature_2m_min[0] };
        let temperature_high = if is_imperial { conversions::celsius_to_fahrenheit(daily.temperature_2m_max[0]) } else { daily.temperature_2m_max[0] };
        let precipitation_sum = if is_imperial { conversions::mm_to_inches(daily.precipitation_sum[0]) } else { daily.precipitation_sum[0] };

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
                precipitation_sum: if is_imperial { conversions::mm_to_inches(daily.precipitation_sum[i]) } else { daily.precipitation_sum[i] },
                precipitation_hours: daily.precipitation_hours[i],
                precipitation_chance: daily.precipitation_probability_max[i],
                temperature_high: if is_imperial { conversions::celsius_to_fahrenheit(daily.temperature_2m_max[i]) } else { daily.temperature_2m_max[i] },
                temperature_low: if is_imperial { conversions::celsius_to_fahrenheit(daily.temperature_2m_min[i]) } else { daily.temperature_2m_min[i] },
            })
            .collect();

        let hourlies: Vec<ContextHourly> = hourly
            .time
            .iter()
            .enumerate()
            .take(24)
            .map(|(i, time)| ContextHourly {
                time: conversions::iso8601_to_time(time.clone()),
                temperature: if is_imperial { conversions::celsius_to_fahrenheit(hourly.temperature_2m[i]) } else { hourly.temperature_2m[i] },
                precipitation_probability: hourly.precipitation_probability[i],
                precipitation: if is_imperial { conversions::mm_to_inches(hourly.precipitation[i]) } else { hourly.precipitation[i] },
            })
            .collect();

        // Calculate precipitation start and end times
        let (precipitation_start, precipitation_end) = Self::calculate_precipitation_timing(&hourly);

        Context {
            city: location.city,
            country: location.country_code,
            temperature,
            temperature_low,
            temperature_high,
            feels_like,
            temperature_unit: if is_imperial { "°F".to_string() } else { "°C".to_string() },
            wind_speed,
            wind_gusts,
            wind_speed_unit: if is_imperial { "mph".to_string() } else { "km/h".to_string() },
            wind_direction: current.wind_direction_10m,
            wind_compass,
            weather_code: current.weather_code,
            weather_icon,
            weather_description,
            openweather_code,
            humidity: current.relative_humidity_2m,
            humidity_unit: "%".to_string(),
            pressure: current.pressure_msl,
            pressure_unit: "hPa".to_string(),
            sunrise,
            sunset,
            uv_index: daily.uv_index_max[0],
            precipitation_chance: daily.precipitation_probability_max[0],
            precipitation_sum,
            precipitation_unit: if is_imperial { "in".to_string() } else { "mm".to_string() },
            precipitation_hours: daily.precipitation_hours[0],
            precipitation_start,
            precipitation_end,
            forecast: dailies,
            hourly: hourlies,

            cache_age,
        }
    }

    /// Calculates when precipitation is expected to start or stop based on hourly data.
    ///
    /// Returns the number of hours until precipitation starts (if currently none) 
    /// or stops (if currently precipitating).
    ///
    /// # Arguments
    ///
    /// * `hourly` - Hourly weather data from API (always in metric)
    ///
    /// # Returns
    ///
    /// Returns a tuple of (precipitation_start, precipitation_end) where:
    /// - precipitation_start: Hours until precipitation starts (if not currently precipitating)
    /// - precipitation_end: Hours until precipitation ends (if currently precipitating)
    /// Both values are None if the condition doesn't occur within the 24-hour forecast.
    fn calculate_precipitation_timing(hourly: &crate::weather::Hourly) -> (Option<i32>, Option<i32>) {
        let mut precipitation_start = None;
        let mut precipitation_end = None;
        
        // Get current precipitation status (first hour)
        let current_precipitation = if hourly.precipitation.is_empty() { 
            0.0 
        } else { 
            hourly.precipitation[0]
        };
        
        let currently_precipitating = current_precipitation > 0.0;
        
        // Look through the next 24 hours (or however many we have)
        for (i, &precip) in hourly.precipitation.iter().enumerate().take(24) {
            let is_precipitating = precip > 0.0;
            
            if !currently_precipitating && is_precipitating && precipitation_start.is_none() {
                // Found when precipitation starts
                precipitation_start = Some(i as i32);
            } else if currently_precipitating && !is_precipitating && precipitation_end.is_none() {
                // Found when precipitation ends
                precipitation_end = Some(i as i32);
            }
            
            // If we've found both conditions or the relevant one, we can break
            if (!currently_precipitating && precipitation_start.is_some()) ||
               (currently_precipitating && precipitation_end.is_some()) {
                break;
            }
        }
        
        (precipitation_start, precipitation_end)
    }
}
