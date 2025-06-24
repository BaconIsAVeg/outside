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
    fn new(context: Context, settings: Settings) -> Self {
        let mut tt = Self::tt();
        let text_template = settings.simple.template.unwrap_or(DEFAULT_TEMPLATE.to_string());

        tt.add_template("text", text_template.as_str()).expect("Failed to add text template");

        let template =
            tt.render("text", &context).unwrap_or_else(|_| "Error rendering text template".to_string());

        SimpleOutput { template }
    }

    fn render(&self) -> String {
        self.template.clone()
    }
}
