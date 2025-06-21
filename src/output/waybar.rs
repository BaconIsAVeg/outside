use crate::context::Context;
use crate::output::Output;

#[derive(serde::Deserialize, Debug)]
pub struct WaybarOutput {
    pub template: String,
}

impl Output for WaybarOutput {
    fn new(context: Context) -> Self {
        let template =
            format!("{} {}{}", context.weather_icon, context.temperature, context.temperature_unit);
        WaybarOutput { template }
    }

    fn render(&self) -> String {
        self.template.clone()
    }
}
