use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct UnitStrings {
    pub temperature: String,
    pub wind_speed: String,
    pub precipitation: String,
}

impl UnitStrings {
    pub fn metric() -> Self {
        UnitStrings {
            temperature: "celsius".to_string(),
            wind_speed: "kmh".to_string(),
            precipitation: "mm".to_string(),
        }
    }

    pub fn imperial() -> Self {
        UnitStrings {
            temperature: "fahrenheit".to_string(),
            wind_speed: "mph".to_string(),
            precipitation: "inch".to_string(),
        }
    }
}
