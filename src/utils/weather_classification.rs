/// Weather classification utilities for categorizing weather conditions by code ranges.

#[derive(Debug, Clone, PartialEq)]
pub enum WeatherCondition {
    Fog,
    Snow,
    Rain,
    Clear,
}

/// Classifies weather conditions based on weather codes.
///
/// This function maps weather codes to general categories used for styling
/// and conditional display logic throughout the application.
///
/// # Arguments
///
/// * `weather_code` - The weather code to classify (typically from Open-Meteo API)
///
/// # Returns
///
/// Returns a `WeatherCondition` enum representing the general weather category.
///
/// # Weather Code Ranges
///
/// - **Fog**: 40-49 (various fog conditions)
/// - **Snow**: 70-79 (snow, snow showers, etc.)
/// - **Rain**: 50-69, 80-99 (rain, drizzle, thunderstorms)
/// - **Clear**: All other codes (clear, partly cloudy, overcast)
pub fn classify_weather(weather_code: i32) -> WeatherCondition {
    match weather_code {
        40..=49 => WeatherCondition::Fog,
        70..=79 => WeatherCondition::Snow,
        50..=69 | 80..=99 => WeatherCondition::Rain,
        _ => WeatherCondition::Clear,
    }
}

/// Checks if the weather condition involves precipitation (rain or snow).
///
/// # Arguments
///
/// * `weather_code` - The weather code to check
///
/// # Returns
///
/// Returns `true` if the weather involves precipitation, `false` otherwise.
pub fn has_precipitation(weather_code: i32) -> bool {
    matches!(classify_weather(weather_code), WeatherCondition::Rain | WeatherCondition::Snow)
}

/// Gets the CSS class name for a weather condition (used in Waybar output).
///
/// # Arguments
///
/// * `weather_code` - The weather code to get the class for
///
/// # Returns
///
/// Returns an `Option<String>` with the CSS class name, or `None` for clear conditions.
pub fn get_weather_css_class(weather_code: i32) -> Option<String> {
    match classify_weather(weather_code) {
        WeatherCondition::Fog => Some("fog".to_string()),
        WeatherCondition::Snow => Some("snow".to_string()),
        WeatherCondition::Rain => Some("rain".to_string()),
        WeatherCondition::Clear => None,
    }
}

/// Gets all applicable CSS classes for weather and temperature conditions.
///
/// This function combines weather condition classes with temperature-based classes
/// to provide a complete set of CSS classes for styling weather displays.
///
/// # Arguments
///
/// * `weather_code` - The weather code to classify
/// * `temperature` - The current temperature
/// * `hot_threshold` - Optional temperature threshold for "hot" class
/// * `cold_threshold` - Optional temperature threshold for "cold" class
///
/// # Returns
///
/// Returns a `Vec<String>` containing all applicable CSS class names.
pub fn get_all_weather_css_classes(
    weather_code: i32,
    temperature: f64,
    hot_threshold: Option<f64>,
    cold_threshold: Option<f64>,
) -> Vec<String> {
    let mut classes = Vec::new();

    // Add temperature-based classes
    if let Some(hot_temp) = hot_threshold {
        if temperature > hot_temp {
            classes.push("hot".to_string());
        }
    }

    if let Some(cold_temp) = cold_threshold {
        if temperature < cold_temp {
            classes.push("cold".to_string());
        }
    }

    // Add weather condition class
    if let Some(weather_class) = get_weather_css_class(weather_code) {
        classes.push(weather_class);
    }

    classes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_weather() {
        assert_eq!(classify_weather(45), WeatherCondition::Fog);
        assert_eq!(classify_weather(75), WeatherCondition::Snow);
        assert_eq!(classify_weather(60), WeatherCondition::Rain);
        assert_eq!(classify_weather(85), WeatherCondition::Rain);
        assert_eq!(classify_weather(0), WeatherCondition::Clear);
        assert_eq!(classify_weather(30), WeatherCondition::Clear);
    }

    #[test]
    fn test_has_precipitation() {
        assert!(!has_precipitation(45)); // fog
        assert!(has_precipitation(75)); // snow
        assert!(has_precipitation(60)); // rain
        assert!(has_precipitation(85)); // rain
        assert!(!has_precipitation(0)); // clear
    }

    #[test]
    fn test_get_weather_css_class() {
        assert_eq!(get_weather_css_class(45), Some("fog".to_string()));
        assert_eq!(get_weather_css_class(75), Some("snow".to_string()));
        assert_eq!(get_weather_css_class(60), Some("rain".to_string()));
        assert_eq!(get_weather_css_class(0), None);
    }

    #[test]
    fn test_get_all_weather_css_classes() {
        // Test with hot temperature and rain
        let classes = get_all_weather_css_classes(60, 35.0, Some(30.0), Some(0.0));
        assert_eq!(classes, vec!["hot", "rain"]);

        // Test with cold temperature and snow
        let classes = get_all_weather_css_classes(75, -5.0, Some(30.0), Some(0.0));
        assert_eq!(classes, vec!["cold", "snow"]);

        // Test with normal temperature and clear weather
        let classes = get_all_weather_css_classes(0, 20.0, Some(30.0), Some(0.0));
        assert!(classes.is_empty());

        // Test with no thresholds
        let classes = get_all_weather_css_classes(45, 20.0, None, None);
        assert_eq!(classes, vec!["fog"]);

        // Test edge case: exactly at threshold
        let classes = get_all_weather_css_classes(60, 30.0, Some(30.0), Some(0.0));
        assert_eq!(classes, vec!["rain"]); // Should not include "hot" for exactly at threshold
    }
}
