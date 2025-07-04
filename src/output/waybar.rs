use crate::context::Context;
use crate::output::Output;
use crate::utils::weather_classification;
use crate::Settings;
use serde::{Deserialize, Serialize};

const DEFAULT_TEXT_TEMPLATE: &str =
    "{weather_icon} {temperature | round}{temperature_unit}{{if precipitation_sum}} 󰖗 {precipitation_chance}%{{endif}}";
const DEFAULT_TOOLTIP_TEMPLATE: &str = "{city}, {country}\n{weather_description}\nFeels Like  {feels_like} {temperature_unit}\nForecast    {temperature_low | round}-{temperature_high| round} {temperature_unit}\nHumidity    {humidity}{humidity_unit}\nPressure    {pressure} {pressure_unit}\nWind        {wind_speed}{wind_gusts} {wind_speed_unit} ({wind_compass})\nPrecip      {precipitation_sum} {precipitation_unit} ({precipitation_chance}% chance)\n\n {sunrise}    {sunset}";

#[derive(Serialize, Deserialize, Debug)]
pub struct WaybarOutput {
    pub text: String,
    pub tooltip: String,
    pub class: Vec<String>,
    pub percentage: i8,
}

impl Output for WaybarOutput {
    /// Creates a new WaybarOutput instance with text, tooltip, and CSS classes.
    ///
    /// Generates Waybar-compatible JSON output with customizable text and tooltip
    /// templates, plus dynamic CSS classes based on weather conditions and
    /// temperature thresholds.
    ///
    /// CSS classes generated:
    /// - "hot" - when temperature exceeds configured hot threshold
    /// - "cold" - when temperature is below configured cold threshold
    /// - Weather condition classes ("fog", "snow", "rain") based on weather codes
    ///   (see utils::weather_classification for specific ranges)
    ///
    /// # Arguments
    ///
    /// * `context` - Weather and location data to be formatted
    /// * `settings` - Settings containing templates and temperature thresholds
    ///
    /// # Returns
    ///
    /// Returns a WaybarOutput instance with formatted text, tooltip, and classes.
    fn new(context: Context, settings: Settings) -> Self {
        let mut tt = Self::tt();
        let text_template = settings.waybar.text.unwrap_or(DEFAULT_TEXT_TEMPLATE.to_string());
        let tooltip_template = settings.waybar.tooltip.unwrap_or(DEFAULT_TOOLTIP_TEMPLATE.to_string());

        tt.add_template("text", text_template.as_str()).expect("Unable to add text template");
        tt.add_template("tooltip", tooltip_template.as_str()).expect("Unable to add tooltip template");

        let text =
            tt.render("text", &context).unwrap_or_else(|_| "Error rendering text template".to_string());
        let tooltip =
            tt.render("tooltip", &context).unwrap_or_else(|_| "Error rendering tooltip template".to_string());

        // Generate all CSS classes using the centralized utility
        let classes = weather_classification::get_all_weather_css_classes(
            context.weather_code,
            context.temperature,
            settings.waybar.hot_temperature,
            settings.waybar.cold_temperature,
        );

        WaybarOutput { text, tooltip, class: classes, percentage: 100 }
    }

    /// Returns the Waybar-compatible JSON output.
    ///
    /// # Returns
    ///
    /// Returns the weather data formatted as JSON for Waybar consumption,
    /// including text, tooltip, CSS classes, and percentage fields.
    fn render(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
