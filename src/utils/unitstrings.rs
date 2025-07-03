use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct UnitStrings {
    pub temperature: String,
    pub wind_speed: String,
    pub precipitation: String,
}

impl UnitStrings {
    /// Creates a UnitStrings instance with metric unit strings.
    ///
    /// Returns unit strings appropriate for metric measurements:
    /// - Temperature: "celsius"
    /// - Wind speed: "kmh" (kilometers per hour)
    /// - Precipitation: "mm" (millimeters)
    ///
    /// # Returns
    ///
    /// Returns a UnitStrings struct configured for metric units.
    pub fn metric() -> Self {
        UnitStrings {
            temperature: "celsius".to_string(),
            wind_speed: "kmh".to_string(),
            precipitation: "mm".to_string(),
        }
    }

    /// Creates a UnitStrings instance with imperial unit strings.
    ///
    /// Returns unit strings appropriate for imperial measurements:
    /// - Temperature: "fahrenheit"
    /// - Wind speed: "mph" (miles per hour)
    /// - Precipitation: "inch" (inches)
    ///
    /// # Returns
    ///
    /// Returns a UnitStrings struct configured for imperial units.
    pub fn imperial() -> Self {
        UnitStrings {
            temperature: "fahrenheit".to_string(),
            wind_speed: "mph".to_string(),
            precipitation: "inch".to_string(),
        }
    }
}
