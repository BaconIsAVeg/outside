use crate::context::Context;
use crate::output::Output;
use serde::{Deserialize, Serialize};
use tinytemplate::TinyTemplate;

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleOutput {
    pub template: String,
}

const DEFAULT_TEMPLATE: &str = "{city}, {country} {weather_icon} {temperature}{temperature_unit}";

impl Output for SimpleOutput {
    fn new(context: Context) -> Self {
        // TODO: Read the template from Settings
        let mut tt = TinyTemplate::new();

        let text_template = DEFAULT_TEMPLATE;
        tt.add_template("text", text_template).expect("Failed to add text template");

        let template =
            tt.render("text", &context).unwrap_or_else(|_| "Error rendering text template".to_string());

        SimpleOutput { template }
    }

    fn render(&self) -> String {
        self.template.clone()
    }
}
