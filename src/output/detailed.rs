use crate::context::Context;
use crate::output::Output;
use crate::Settings;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DetailedOutput {
    pub template: String,
}

impl Output for DetailedOutput {
    /// Creates a new DetailedOutput instance with rendered template.
    ///
    /// Processes the context data through a comprehensive template that displays
    /// current weather conditions, atmospheric data, and a 7-day forecast.
    /// Uses a fixed template for consistent detailed output format.
    ///
    /// # Arguments
    ///
    /// * `context` - Weather and location data to be formatted
    /// * `_` - Settings parameter (unused for detailed output)
    ///
    /// # Returns
    ///
    /// Returns a DetailedOutput instance with the rendered template.
    fn new(context: Context, _: Settings) -> Self {
        let mut tt = Self::tt();

        // Build dynamic template with precipitation timing
        let mut template_parts = vec![
            "{city}, {country}".to_string(),
            "    Current:     {temperature}{temperature_unit} {weather_description}".to_string(),
            "    Feels Like:  {feels_like}{temperature_unit}".to_string(),
            "    Humidity:    {humidity}{humidity_unit}".to_string(),
            "    Pressure:    {pressure}{pressure_unit}".to_string(),
            "    Wind:        {wind_speed}{wind_speed_unit} with gusts up to {wind_gusts}{wind_speed_unit} ({wind_compass})".to_string(),
            "    UV Index:    {uv_index}".to_string(),
        ];
        // Add precipitation with optional timing description
        let precip_line = if let Some(description) = &context.precipitation_description {
            format!("    Precip:      {{precipitation_sum}} {{precipitation_unit}} ({{precipitation_chance}}% chance, {description})")
        } else {
            "    Precip:      {precipitation_sum} {precipitation_unit} ({precipitation_chance}% chance)"
                .to_string()
        };
        template_parts.push(precip_line);

        template_parts.push("    Sunrise:     {sunrise}".to_string());
        template_parts.push("    Sunset:      {sunset}".to_string());
        template_parts.push("".to_string());
        template_parts.push("    {{ for day in forecast -}}".to_string());
        template_parts.push("    {day.date}    {day.temperature_low | round}-{day.temperature_high | round}{temperature_unit} - {day.weather_description}".to_string());
        template_parts.push("    {{ endfor }}".to_string());

        let text_template = template_parts.join("\n");
        tt.add_template("text", &text_template).expect("Failed to add text template");

        let template =
            tt.render("text", &context).unwrap_or_else(|_| "Error rendering text template".to_string());

        DetailedOutput { template }
    }

    /// Returns the rendered detailed weather output.
    ///
    /// # Returns
    ///
    /// Returns the detailed weather output as a multi-line string with
    /// current conditions and forecast information.
    fn render(&self) -> String {
        self.template.clone()
    }
}
