use crate::context::Context;
use crate::output::Output;
use crate::Settings;
use serde::{Deserialize, Serialize};
use tinytemplate::TinyTemplate;

const DEFAULT_TEMPLATE: &str = "{city}, {country} {weather_icon} {temperature}{temperature_unit}";

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleOutput {
    pub template: String,
}

impl Output for SimpleOutput {
    fn new(context: Context, settings: Settings) -> Self {
        let mut tt = TinyTemplate::new();
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
