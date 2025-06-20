pub fn weather_code_to_icon(code: i32) -> String {
    owm_weather_icon(meteo_to_owm_code_map(code))
}

pub fn owm_weather_icon(condition: String) -> String {
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

pub fn meteo_to_owm_code_map(code: i32) -> String {
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
