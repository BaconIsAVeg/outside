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
    pub precipitation_description: Option<String>,
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
    pub weather_code: i32,
    pub weather_icon: String,
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

        let sunrise = conversions::iso8601_to_time(daily.sunrise[0].clone(), settings.hour24);
        let sunset = conversions::iso8601_to_time(daily.sunset[0].clone(), settings.hour24);

        let cache_age = now - weather.created_at;

        // Convert values based on user settings
        let is_imperial = settings.units == Units::Imperial;

        // Convert current weather values
        let temperature = if is_imperial {
            conversions::celsius_to_fahrenheit(current.temperature_2m)
        } else {
            current.temperature_2m
        };
        let feels_like = if is_imperial {
            conversions::celsius_to_fahrenheit(current.apparent_temperature)
        } else {
            current.apparent_temperature
        };
        let wind_speed = if is_imperial {
            conversions::kmh_to_mph(current.wind_speed_10m)
        } else {
            current.wind_speed_10m
        };
        let wind_gusts = if is_imperial {
            conversions::kmh_to_mph(current.wind_gusts_10m)
        } else {
            current.wind_gusts_10m
        };

        // Convert daily values for today
        let temperature_low = if is_imperial {
            conversions::celsius_to_fahrenheit(daily.temperature_2m_min[0])
        } else {
            daily.temperature_2m_min[0]
        };
        let temperature_high = if is_imperial {
            conversions::celsius_to_fahrenheit(daily.temperature_2m_max[0])
        } else {
            daily.temperature_2m_max[0]
        };
        let precipitation_sum = if is_imperial {
            conversions::mm_to_inches(daily.precipitation_sum[0])
        } else {
            daily.precipitation_sum[0]
        };

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
                precipitation_sum: if is_imperial {
                    conversions::mm_to_inches(daily.precipitation_sum[i])
                } else {
                    daily.precipitation_sum[i]
                },
                precipitation_hours: daily.precipitation_hours[i],
                precipitation_chance: daily.precipitation_probability_max[i],
                temperature_high: if is_imperial {
                    conversions::celsius_to_fahrenheit(daily.temperature_2m_max[i])
                } else {
                    daily.temperature_2m_max[i]
                },
                temperature_low: if is_imperial {
                    conversions::celsius_to_fahrenheit(daily.temperature_2m_min[i])
                } else {
                    daily.temperature_2m_min[i]
                },
            })
            .collect();

        // Find the current hour index to start hourly forecast from current time
        let current_hour_index = Self::find_current_hour_index(&hourly.time, now, weather.utc_offset_seconds);

        let hourlies: Vec<ContextHourly> = hourly
            .time
            .iter()
            .enumerate()
            .skip(current_hour_index)
            .take(24)
            .map(|(i, time)| ContextHourly {
                time: conversions::iso8601_to_time(time.clone(), settings.hour24),
                temperature: if is_imperial {
                    conversions::celsius_to_fahrenheit(hourly.temperature_2m[i])
                } else {
                    hourly.temperature_2m[i]
                },
                precipitation_probability: hourly.precipitation_probability[i],
                precipitation: if is_imperial {
                    conversions::mm_to_inches(hourly.precipitation[i])
                } else {
                    hourly.precipitation[i]
                },
                weather_code: hourly.weather_code[i],
                weather_icon: mappings::weather_code2icon(hourly.weather_code[i]),
            })
            .collect();

        // Calculate precipitation start and end times (accounting for current hour offset)
        let (precipitation_start, precipitation_end) =
            Self::calculate_precipitation_timing(hourly, current_hour_index);

        // Create precipitation description
        let precipitation_description = Self::create_precipitation_description(
            precipitation_start,
            precipitation_end,
            hourly,
            current_hour_index,
        );

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
            precipitation_description,
            forecast: dailies,
            hourly: hourlies,

            cache_age,
        }
    }

    /// Finds the index of the current hour in the hourly time array.
    ///
    /// This function converts the current UTC timestamp to the location's timezone,
    /// then finds the hourly entry that corresponds to the current local time.
    /// This is used to start the 24-hour forecast from the current time rather than from midnight.
    ///
    /// # Arguments
    ///
    /// * `hourly_times` - Array of ISO8601 datetime strings from the API (in location's timezone)
    /// * `current_timestamp` - Current Unix timestamp in seconds (UTC)
    /// * `utc_offset_seconds` - UTC offset for the location's timezone
    ///
    /// # Returns
    ///
    /// Returns the index of the hour closest to the current local time, or 0 if no match is found.
    fn find_current_hour_index(
        hourly_times: &[String],
        current_timestamp: u64,
        utc_offset_seconds: i32,
    ) -> usize {
        use chrono::{FixedOffset, NaiveDateTime, TimeZone};

        // Create timezone offset from the location's UTC offset
        let timezone = FixedOffset::east_opt(utc_offset_seconds).unwrap_or(FixedOffset::east_opt(0).unwrap());

        // Convert current UTC timestamp to the location's timezone
        let current_local = timezone.timestamp_opt(current_timestamp as i64, 0).unwrap();

        for (i, time_str) in hourly_times.iter().enumerate() {
            if let Ok(hour_dt) = NaiveDateTime::parse_from_str(time_str, "%Y-%m-%dT%H:%M") {
                let hour_local = timezone.from_local_datetime(&hour_dt).unwrap();

                // If this hour is at or after the current local time, use this index
                if hour_local >= current_local {
                    return i;
                }
            }
        }

        // If no hour found at or after current time, start from beginning
        0
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
    ///   Both values are None if the condition doesn't occur within the 24-hour forecast.
    fn calculate_precipitation_timing(
        hourly: &crate::weather::Hourly,
        current_hour_index: usize,
    ) -> (Option<i32>, Option<i32>) {
        let mut precipitation_start = None;
        let mut precipitation_end = None;

        // Get current precipitation status (at current hour index)
        let current_precipitation = if hourly.precipitation.len() > current_hour_index {
            hourly.precipitation[current_hour_index]
        } else {
            0.0
        };

        let currently_precipitating = current_precipitation > 0.0;

        // Look through the next 24 hours starting from current hour
        for (i, &precip) in hourly.precipitation.iter().enumerate().skip(current_hour_index).take(24) {
            let is_precipitating = precip > 0.0;
            let hours_from_now = (i - current_hour_index) as i32;

            if !currently_precipitating && is_precipitating && precipitation_start.is_none() {
                // Found when precipitation starts
                precipitation_start = Some(hours_from_now);
            } else if currently_precipitating && !is_precipitating && precipitation_end.is_none() {
                // Found when precipitation ends
                precipitation_end = Some(hours_from_now);
            }

            // If we've found both conditions or the relevant one, we can break
            if (!currently_precipitating && precipitation_start.is_some())
                || (currently_precipitating && precipitation_end.is_some())
            {
                break;
            }
        }

        (precipitation_start, precipitation_end)
    }

    /// Creates a human-readable description of precipitation timing.
    ///
    /// # Arguments
    ///
    /// * `precipitation_start` - Hours until precipitation starts (if not currently precipitating)
    /// * `precipitation_end` - Hours until precipitation ends (if currently precipitating)
    /// * `hourly` - Hourly weather data to determine current precipitation status
    ///
    /// # Returns
    ///
    /// Returns an Option<String> with a description like "Starts in 6 hours" or "Stops in 2 hours"
    /// Returns None if no precipitation timing is available.
    fn create_precipitation_description(
        precipitation_start: Option<i32>,
        precipitation_end: Option<i32>,
        hourly: &crate::weather::Hourly,
        current_hour_index: usize,
    ) -> Option<String> {
        // Determine current precipitation status (at current hour index)
        let current_precipitation = if hourly.precipitation.len() > current_hour_index {
            hourly.precipitation[current_hour_index]
        } else {
            0.0
        };
        let currently_precipitating = current_precipitation > 0.0;

        if currently_precipitating {
            // Show when precipitation will end
            if let Some(hours) = precipitation_end {
                let hour_text = if hours == 1 { "hour" } else { "hours" };
                Some(format!("Stops in {hours} {hour_text}"))
            } else {
                None
            }
        } else {
            // Show when precipitation will start
            if let Some(hours) = precipitation_start {
                let hour_text = if hours == 1 { "hour" } else { "hours" };
                Some(format!("Starts in {hours} {hour_text}"))
            } else {
                None
            }
        }
    }
}
