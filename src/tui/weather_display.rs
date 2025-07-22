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
        let mut info = format!(
            "Temperature:     {}{}\n\
            Humidity:        {}%\n\
            Pressure:        {} hPa\n\
            Wind:            {} {} with gusts up to {} {} ({})\n\
            UV Index:        {}\n\
            Precipitation:   {} {} ({}% chance)",
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
            context.precipitation_chance
        );

        // Add precipitation timing if available
        if let Some(description) = &context.precipitation_description {
            info.push('\n');
            info.push_str(&format!("                 {description}"));
        }

        info.push_str(&format!("\nSun:             {} • {}", context.sunrise, context.sunset));

        info
    }

    pub fn format_hourly_forecast(context: &Context) -> String {
        // Calculate available width: assume 80 chars wide terminal minus location panel
        let available_width = Self::calculate_available_forecast_width();
        Self::format_hourly_forecast_with_width(context, available_width)
    }

    fn calculate_available_forecast_width() -> usize {
        // Get actual terminal width using termsize crate
        let terminal_width: usize = match termsize::get() {
            Some(size) => size.cols as usize,
            None => 120, // Fallback for wide terminals if detection fails
        };

        // Account for location panel and margins
        // Location panel: LOCATION_LIST_WIDTH (24) + borders/spacing (~6)
        // Weather panel borders: ~4 chars
        let location_panel_width: usize = crate::tui::constants::LOCATION_LIST_WIDTH + 6;
        let weather_panel_margins: usize = 4;
        let used_width = location_panel_width + weather_panel_margins;

        (terminal_width.saturating_sub(used_width)).max(40) // Minimum 40 chars
    }

    pub fn format_hourly_forecast_with_width(context: &Context, available_width: usize) -> String {
        let mut forecast_text = String::new();

        // Fixed layout: 3 columns, 8 rows, but adjust cell width based on available space
        let num_cols = 3;
        let num_rows = 8;
        let col_spacing = 4; // Space between columns
        let total_spacing = (num_cols - 1) * col_spacing;
        let cell_width = (available_width.saturating_sub(total_spacing)) / num_cols;

        // Display 24 hours in 3 columns with 8 rows each
        for row in 0..num_rows {
            let mut line = String::new();

            for col in 0..num_cols {
                let hour_index = col * num_rows + row;
                if hour_index < context.hourly.len() {
                    let hour = &context.hourly[hour_index];

                    // Format: " 9am 󰖖 22°C  0.1mm ( 84%)" for 12-hour or "19:00 󰖖 22°C  0.1mm ( 84%)" for 24-hour
                    let formatted_time = if hour.time.contains("am") || hour.time.contains("pm") {
                        // 12-hour format: convert "09:00am" to " 9am"
                        let time_without_zeros = hour.time.replace(":00", "");
                        if let Some(stripped) = time_without_zeros.strip_prefix('0') {
                            format!(" {stripped}")
                        } else {
                            format!("{time_without_zeros:>4}")
                        }
                    } else {
                        // 24-hour format: keep as is "19:00"
                        format!("{:>5}", hour.time)
                    };

                    let temp_unit = if context.temperature_unit.contains('F') { "F" } else { "C" };
                    let temp = format!("{:2}°{}", hour.temperature.round(), temp_unit);
                    let precip = format!("{:4.1}{}", hour.precipitation, context.precipitation_unit);
                    let prob = format!("{:3}%", hour.precipitation_probability);

                    let cell_content =
                        format!("{formatted_time} {} {temp} {precip} {prob}", hour.weather_icon);
                    let padded_cell = format!("{cell_content:<cell_width$}");
                    line.push_str(&padded_cell);

                    // Add column separator except for last column
                    if col < 2 {
                        line.push_str(&" ".repeat(col_spacing));
                    }
                } else {
                    // Fill with spaces if we run out of hours
                    line.push_str(&" ".repeat(cell_width));
                    if col < 2 {
                        line.push_str(&" ".repeat(col_spacing));
                    }
                }
            }

            forecast_text.push_str(&line);
            forecast_text.push('\n');
        }

        forecast_text
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
                "{:10} {}  {:>2}-{:<2}{}  {}\n",
                display_date,
                day.weather_icon,
                day.temperature_low.round(),
                day.temperature_high.round(),
                context.temperature_unit,
                weather_description
            ));
        }
        forecast_text.push('\n');
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
