const COMPASS_DIRECTIONS: [&str; 9] = ["N", "NE", "E", "SE", "S", "SW", "W", "NW", "N"];
const COMPASS_SEGMENT: f64 = 45.0;

/// Converts wind direction degrees to compass direction string.
///
/// Takes a wind direction in degrees (0-360) and returns the corresponding
/// compass direction (N, NE, E, SE, S, SW, W, NW). The function handles
/// wraparound and uses 45-degree segments for each direction.
///
/// # Arguments
///
/// * `degrees` - Wind direction in degrees (0-360, where 0/360 = North)
///
/// # Returns
///
/// Returns a string representing the compass direction, or "??" if invalid.
pub fn degrees2compass(degrees: f64) -> String {
    let normalized = degrees % 360.0;
    let index = ((normalized) / COMPASS_SEGMENT).round() as i32;

    match COMPASS_DIRECTIONS.get(index as usize) {
        Some(&direction) => direction,
        None => "??",
    }
    .to_string()
}

/// Converts a weather code to a weather icon string.
///
/// Takes a weather code from the Open-Meteo API and returns the corresponding
/// Unicode weather icon. This is a convenience function that combines
/// weather code mapping with icon lookup.
///
/// # Arguments
///
/// * `code` - Weather code from Open-Meteo API
///
/// # Returns
///
/// Returns a Unicode string representing the weather icon.
pub fn weather_code2icon(code: i32) -> String {
    openweather_weather_icon(meteo2openweather_codes(code))
}

/// Maps OpenWeather API condition codes to Unicode weather icons.
///
/// Takes an OpenWeather condition code (e.g., "01d", "10n") and returns
/// the corresponding Unicode weather icon. Supports both day and night
/// variants for most conditions.
///
/// # Arguments
///
/// * `condition` - OpenWeather condition code string
///
/// # Returns
///
/// Returns a Unicode weather icon string, or empty string if unknown.
pub fn openweather_weather_icon(condition: String) -> String {
    match condition.as_str() {
        "01d" => "󰖙",
        "01n" => "󰖔",
        "02d" | "02n" => "󰖕",
        "03d" | "03n" => "󰖐",
        "04d" | "04n" => "󰖐",
        "09d" | "09n" => "󰖗",
        "10d" | "10n" => "󰖖",
        "11d" | "11n" => "󰖓",
        "13d" | "13n" => "󰼶",
        "50d" | "50n" => "",
        _ => "",
    }
    .to_string()
}

/// Converts Open-Meteo weather codes to OpenWeather API equivalent codes.
///
/// Maps weather codes from the Open-Meteo API to their OpenWeather API
/// equivalents for consistent icon and description handling. This enables
/// using established OpenWeather icon sets with Open-Meteo data.
///
/// # Arguments
///
/// * `code` - Weather code from Open-Meteo API
///
/// # Returns
///
/// Returns the equivalent OpenWeather condition code, or "unknown" if unmapped.
pub fn meteo2openweather_codes(code: i32) -> String {
    match code {
        0 => "01d",       // Clear sky
        1 => "02d",       // Mainly clear
        2 => "03d",       // Partly cloudy
        3 => "04d",       // Overcast
        45 => "50d",      // Fog
        48 => "50d",      // Depositing rime fog
        51 => "09d",      // Drizzle, light
        53 => "09d",      // Drizzle, moderate
        55 => "09d",      // Drizzle, dense
        56 => "09n",      // Freezing drizzle, light
        57 => "09n",      // Freezing drizzle, dense
        61 => "10d",      // Rain, slight
        63 => "10d",      // Rain, moderate
        65 => "10d",      // Rain, heavy
        66 => "13n",      // Freezing rain, light
        67 => "13n",      // Freezing rain, heavy
        71 => "13d",      // Snow fall, slight
        73 => "13d",      // Snow fall, moderate
        75 => "13d",      // Snow fall, heavy
        77 => "13d",      // Snow grains
        80 => "09d",      // Rain showers, slight
        81 => "09d",      // Rain showers, moderate or heavy
        82 => "09d",      // Heavy rain showers
        85 => "13n",      // Snow showers slight to moderate
        86 => "13n",      // Snow showers heavy
        95 => "11d",      // Thunderstorm
        96 | 99 => "11n", // Thunderstorm with hail
        _ => "unknown",
    }
    .to_string()
}

/// Converts a weather code to a human-readable description.
///
/// Takes a weather code from the Open-Meteo API and returns a descriptive
/// string explaining the weather condition in plain English.
///
/// # Arguments
///
/// * `code` - Weather code from Open-Meteo API
///
/// # Returns
///
/// Returns a human-readable weather description string.
pub fn weather_description(code: i32) -> String {
    match code {
        0 => "Clear sky",
        1 => "Mainly clear",
        2 => "Partly cloudy",
        3 => "Overcast",
        45 => "Fog",
        48 => "Depositing rime fog",
        51 => "Drizzle, light",
        53 => "Drizzle, moderate",
        55 => "Drizzle, dense",
        56 => "Freezing drizzle, light",
        57 => "Freezing drizzle, dense",
        61 => "Rain, slight",
        63 => "Rain, moderate",
        65 => "Rain, heavy",
        66 => "Freezing rain, light",
        67 => "Freezing rain, heavy",
        71 => "Snow fall, slight",
        73 => "Snow fall, moderate",
        75 => "Snow fall, heavy",
        77 => "Snow grains",
        80 => "Rain showers, slight",
        81 => "Rain showers, moderate or heavy",
        82 => "Heavy rain showers",
        85 => "Snow showers slight to moderate",
        86 => "Snow showers heavy",
        95 => "Thunderstorm",
        96 | 99 => "Thunderstorm with hail",
        _ => "Unknown weather code",
    }
    .to_string()
}
