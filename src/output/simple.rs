use crate::context::Context;
use crate::output::Output;
use crate::Settings;
use serde::{Deserialize, Serialize};

const DEFAULT_TEMPLATE: &str =
    "{weather_description} {temperature | round}{temperature_unit} | Wind {wind_speed | round}{wind_gusts | round}{{if precipitation_chance}} | Precipitation {precipitation_chance}%{{endif}}";

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleOutput {
    pub template: String,
}

impl Output for SimpleOutput {
    /// Creates a new SimpleOutput instance with rendered template.
    ///
    /// Processes the context data through a customizable template to produce
    /// a concise, single-line weather summary suitable for status bars and
    /// simple displays.
    ///
    /// # Arguments
    ///
    /// * `context` - Weather and location data to be formatted
    /// * `settings` - Settings containing the optional custom template
    ///
    /// # Returns
    ///
    /// Returns a SimpleOutput instance with the rendered template.
    fn new(context: Context, settings: Settings) -> Self {
        let mut tt = Self::tt();
        let text_template = settings.simple.template.unwrap_or(DEFAULT_TEMPLATE.to_string());

        tt.add_template("text", text_template.as_str()).expect("Failed to add text template");

        let template =
            tt.render("text", &context).unwrap_or_else(|_| "Error rendering text template".to_string());

        SimpleOutput { template }
    }

    /// Returns the rendered simple weather output.
    ///
    /// # Returns
    ///
    /// Returns the simple weather output as a single-line string.
    fn render(&self) -> String {
        self.template.clone()
    }
}
