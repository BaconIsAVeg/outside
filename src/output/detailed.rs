use crate::context::Context;
use crate::output::Output;

#[derive(serde::Deserialize, Debug)]
pub struct DetailedOutput {
    pub template: String,
}

impl Output for DetailedOutput {
    fn new(context: Context) -> Self {
        let template =
            format!("{} {}{}", context.weather_icon, context.temperature, context.temperature_unit);
        DetailedOutput { template }
    }

    fn render(&self) -> String {
        self.template.clone()
    }
}
