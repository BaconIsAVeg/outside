use crate::context::Context;
use crate::output::Output;
use crate::Settings;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DetailedOutput {
    pub template: String,
}

const DEFAULT_TEMPLATE: &str = "{city}, {country}\n\
    Current:     {temperature}{temperature_unit} {weather_description}\n\
    Feels Like:  {feels_like}{temperature_unit}\n\
    Humidity:    {humidity}{humidity_unit}\n\
    Pressure:    {pressure}{pressure_unit}\n\
    Wind:        {wind_speed}{wind_speed_unit} with gusts up to {wind_gusts}{wind_speed_unit} ({wind_compass})\n\
    UV Index:    {uv_index}\n\
    Precip:      {precipitation_sum} {precipitation_unit} ({precipitation_chance}% chance)\n\
    Sunrise:     {sunrise}\n\
    Sunset:      {sunset}\n\n\
    {{ for day in forecast -}}
    {day.date}    {day.temperature_low | round}-{day.temperature_high | round}{temperature_unit} - {day.weather_description}\n\
    {{ endfor }}";
impl Output for DetailedOutput {
    fn new(context: Context, _: Settings) -> Self {
        let mut tt = Self::tt();
        let text_template = DEFAULT_TEMPLATE;
        tt.add_template("text", text_template).expect("Failed to add text template");

        let template =
            tt.render("text", &context).unwrap_or_else(|_| "Error rendering text template".to_string());

        DetailedOutput { template }
    }

    fn render(&self) -> String {
        self.template.clone()
    }
}
