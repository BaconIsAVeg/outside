use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Units {
    pub temperature: String,
    pub wind_speed: String,
    pub precipitation: String,
}

impl Units {
    pub fn metric() -> Self {
        Units {
            temperature: "celsius".to_string(),
            wind_speed: "kmh".to_string(),
            precipitation: "mm".to_string(),
        }
    }

    pub fn imperial() -> Self {
        Units {
            temperature: "fahrenheit".to_string(),
            wind_speed: "mph".to_string(),
            precipitation: "inch".to_string(),
        }
    }
}
