use crate::context::Context;
use crate::output::Output;

#[derive(serde::Deserialize, Debug)]
pub struct SimpleOutput {
    pub template: String,
}

impl Output for SimpleOutput {
    fn new(context: Context) -> Self {
        let template =
            format!("{} {}{}", context.weather_icon, context.temperature, context.temperature_unit);
        SimpleOutput { template }
    }

    fn render(&self) -> String {
        self.template.clone()
    }
}
