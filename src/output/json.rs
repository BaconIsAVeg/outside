use crate::context::Context;
use crate::output::Output;
use crate::Settings;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonOutput {
    pub template: String,
}

impl Output for JsonOutput {
    /// Creates a new JsonOutput instance with serialized context data.
    ///
    /// Converts the entire context structure to JSON format, providing
    /// access to all weather data fields for programmatic consumption.
    ///
    /// # Arguments
    ///
    /// * `context` - Weather and location data to be serialized
    /// * `_` - Settings parameter (unused for JSON output)
    ///
    /// # Returns
    ///
    /// Returns a JsonOutput instance with the serialized context data.
    fn new(context: Context, _: Settings) -> Self {
        let template = serde_json::to_string(&context)
            .unwrap_or_else(|_| "{{\"error\": \"Unable to serialize Context\"}}".to_string());
        JsonOutput { template }
    }

    /// Returns the JSON-formatted weather output.
    ///
    /// # Returns
    ///
    /// Returns the complete context data as a JSON string.
    fn render(&self) -> String {
        self.template.clone()
    }
}
