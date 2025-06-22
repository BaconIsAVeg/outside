use crate::context::Context;
use crate::output::Output;
use serde::{Deserialize, Serialize};
use tinytemplate::TinyTemplate;

#[derive(Serialize, Deserialize, Debug)]
pub struct WaybarOutput {
    pub text: String,
    pub tooltip: String,
    pub class: Vec<String>,
    pub percentage: i8,
}

const DEFAULT_TEXT_TEMPLATE: &str =
    "{weather_icon} {temperature_round}{temperature_unit} 󰖝 {wind_speed_round}{wind_gusts_round}";
const DEFAULT_TOOLTIP_TEMPLATE: &str = "{city}, {country}\n{weather_description}\nFeels like: {feels_like} {temperature_unit}\nHumidity: {humidity}{humidity_unit}\nPressure: {pressure} {pressure_unit}\nWind: {wind_speed}{wind_gusts} {wind_speed_unit} ({wind_compass})\nPrecipitation: {precipitation_sum} {precipitation_unit} ({precipitation_chance}%)\n\n {sunrise}  {sunset}";
impl Output for WaybarOutput {
    fn new(context: Context) -> Self {
        let mut tt = TinyTemplate::new();
        let text_template = DEFAULT_TEXT_TEMPLATE;
        let tooltip_template = DEFAULT_TOOLTIP_TEMPLATE;

        tt.add_template("text", text_template).expect("Unable to add text template");
        tt.add_template("tooltip", tooltip_template).expect("Unable to add tooltip template");

        // TODO: Add the hot/cold/inclement weather classes
        let text =
            tt.render("text", &context).unwrap_or_else(|_| "Error rendering text template".to_string());
        let tooltip =
            tt.render("tooltip", &context).unwrap_or_else(|_| "Error rendering tooltip template".to_string());

        WaybarOutput { text, tooltip, class: vec!["".to_string()], percentage: 100 }
    }

    fn render(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
