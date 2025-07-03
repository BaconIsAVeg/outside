pub mod detailed;
pub mod json;
pub mod simple;
pub mod tui;
pub mod waybar;

use crate::context::Context;
use crate::Settings;

use std::fmt::Write;
use tinytemplate::TinyTemplate;

/// Trait for different weather output formats.
///
/// This trait defines the interface for converting weather context data into
/// formatted output strings. Each output format implements this trait to provide
/// its own rendering logic and template system.
pub trait Output {
    /// Creates a new output instance with the given context and settings.
    ///
    /// # Arguments
    ///
    /// * `context` - Weather and location data to be formatted
    /// * `settings` - User configuration including templates and preferences
    ///
    /// # Returns
    ///
    /// Returns a new instance of the output formatter configured with the provided data.
    fn new(context: Context, settings: Settings) -> Self;

    /// Renders the output as a formatted string.
    ///
    /// # Returns
    ///
    /// Returns the formatted weather information as a string ready for display.
    fn render(&self) -> String;

    /// Returns a configured TinyTemplate instance with custom formatters.
    ///
    /// Sets up the template engine with custom formatters like the `round` filter
    /// for formatting numeric values in templates.
    ///
    /// # Returns
    ///
    /// Returns a TinyTemplate instance ready for use with weather data.
    fn tt() -> TinyTemplate<'static> {
        let mut tt = TinyTemplate::new();
        tt.add_formatter("round", |value, output| {
            write!(output, "{:.0}", value.as_f64().unwrap_or(0.0).round())?;
            Ok(())
        });
        tt
    }
}

/// Generic function to render weather data using any output format.
///
/// This function provides a polymorphic way to render weather data by accepting
/// any type that implements the `Output` trait. It creates an instance of the
/// specified output format and renders it to a string.
///
/// # Type Parameters
///
/// * `O` - The output format type that implements the `Output` trait
///
/// # Arguments
///
/// * `context` - Weather and location data to be formatted
/// * `settings` - User configuration including templates and preferences
///
/// # Returns
///
/// Returns the formatted weather information as a string.
pub fn render_output<O: Output>(context: Context, settings: Settings) -> String {
    let output = O::new(context, settings);
    output.render()
}
