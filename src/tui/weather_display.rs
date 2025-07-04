use crate::context::Context;
use crate::utils::weather_classification;

pub struct WeatherDisplay;

impl WeatherDisplay {
    pub fn format_header_text(context: &Context) -> String {
        format!(
            "{}, {}\n\
            {} {}{} • {} • Feels like {}{}",
            context.city,
            context.country,
            context.weather_icon,
            context.temperature.round(),
            context.temperature_unit,
            context.weather_description,
            context.feels_like.round(),
            context.temperature_unit
        )
    }

    pub fn format_current_info(context: &Context) -> String {
        format!(
            "Temperature:     {}{}\n\
            Humidity:        {}%\n\
            Pressure:        {} hPa\n\
            Wind:            {} {} with gusts up to {} {} ({})\n\
            UV Index:        {}\n\
            Precipitation:   {} {} ({}% chance)\n\
            Sun:             {} • {}",
            context.temperature.round(),
            context.temperature_unit,
            context.humidity,
            context.pressure,
            context.wind_speed.round(),
            context.wind_speed_unit,
            context.wind_gusts.round(),
            context.wind_speed_unit,
            context.wind_compass,
            context.uv_index,
            context.precipitation_sum,
            context.precipitation_unit,
            context.precipitation_chance,
            context.sunrise,
            context.sunset
        )
    }

    pub fn format_forecast_text(context: &Context) -> String {
        let mut forecast_text = String::new();
        for (index, day) in context.forecast.iter().enumerate() {
            let display_date = if index == 0 {
                "Today".to_string()
            } else if index == 1 {
                "Tomorrow".to_string()
            } else {
                day.date.clone()
            };
            let weather_description = if weather_classification::has_precipitation(day.weather_code) {
                format!("{} ({}%)", day.weather_description, day.precipitation_chance)
            } else {
                day.weather_description.clone()
            };

            forecast_text.push_str(&format!(
                "{:10} {} {:>2}-{:<2}{} {}\n",
                display_date,
                day.weather_icon,
                day.temperature_low.round(),
                day.temperature_high.round(),
                context.temperature_unit,
                weather_description
            ));
        }
        forecast_text
    }

    pub fn format_loading_message() -> String {
        "Loading weather data...".to_string()
    }

    pub fn format_wait_message() -> String {
        "Please wait...".to_string()
    }

    pub fn format_units_switching_message() -> String {
        "Switching units...".to_string()
    }
}
