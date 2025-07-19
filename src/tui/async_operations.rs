use crate::api::location::LocationData;
use crate::api::weather::Weather;
use crate::context::Context;
use crate::tui::constants::*;
use crate::tui::state_manager::TuiStateManager;
use crate::tui::weather_display::WeatherDisplay;
use cursive::views::{ProgressBar, TextView};
use cursive::Cursive;
use std::thread;

pub struct WeatherFetcher {
    state_manager: TuiStateManager,
}

impl WeatherFetcher {
    pub fn new(state_manager: TuiStateManager) -> Self {
        Self { state_manager }
    }

    pub fn fetch_and_update<F, E>(
        &self,
        location: String,
        siv: &mut Cursive,
        success_callback: F,
        error_callback: E,
    ) where
        F: Fn(&mut Cursive, &TuiStateManager, Context) + Send + 'static,
        E: Fn(&mut Cursive, &TuiStateManager, String) + Send + 'static,
    {
        // Set loading state
        self.state_manager.set_loading(true);

        // Update display to show loading
        self.update_ui_loading(siv);

        // Get a handle to the cursive instance for async update
        let cb_sink = siv.cb_sink().clone();
        let location_clone = location.clone();
        let state_manager_clone = self.state_manager.clone();

        // Spawn background thread to fetch weather data
        thread::spawn(move || {
            if let Ok(result) = Self::fetch_weather_for_location(&location_clone, &state_manager_clone) {
                // Send callback to update UI on main thread
                cb_sink
                    .send(Box::new(move |s| {
                        // Note: we don't update the currently_selected_location here since success_callback will handle it
                        state_manager_clone.update_context(result.clone());
                        success_callback(s, &state_manager_clone, result);
                    }))
                    .unwrap();
            } else {
                // Handle error case
                let error_message = format!("Failed to fetch location data for: {}", location_clone);
                cb_sink
                    .send(Box::new(move |s| {
                        state_manager_clone.set_loading(false);
                        error_callback(s, &state_manager_clone, error_message);
                    }))
                    .unwrap();
            }
        });
    }

    pub fn switch_location(&self, siv: &mut Cursive, location: String) {
        let location_clone = location.clone();
        self.fetch_and_update(
            location,
            siv,
            move |s, state_manager, context| {
                // Update both context and currently selected location
                state_manager.update_context_with_location(context, location_clone.clone());
                Self::update_weather_display(s, state_manager);
            },
            |s, _state_manager, error_message| {
                Self::show_error_dialog(s, &error_message);
            },
        );
    }

    pub fn toggle_units(&self, siv: &mut Cursive) {
        let current_location = self.state_manager.get_current_location();
        self.state_manager.toggle_units();
        self.state_manager.set_loading(true);

        // Update display to show units switching
        siv.call_on_name(WEATHER_HEADER_NAME, |view: &mut TextView| {
            view.set_content(WeatherDisplay::format_units_switching_message());
        });
        siv.call_on_name(WEATHER_CURRENT_NAME, |view: &mut TextView| {
            view.set_content(WeatherDisplay::format_wait_message());
        });
        siv.call_on_name(WEATHER_FORECAST_NAME, |view: &mut TextView| {
            view.set_content("");
        });

        let cb_sink = siv.cb_sink().clone();
        let state_manager_clone = self.state_manager.clone();

        thread::spawn(move || {
            if let Ok(result) = Self::fetch_weather_for_location(&current_location, &state_manager_clone) {
                let location_for_update = current_location.clone();
                cb_sink
                    .send(Box::new(move |s| {
                        state_manager_clone.update_context_with_location(result, location_for_update);
                        Self::update_weather_display(s, &state_manager_clone);
                    }))
                    .unwrap();
            } else {
                cb_sink
                    .send(Box::new(move |s| {
                        state_manager_clone.set_loading(false);
                        Self::show_error_dialog(s, "Failed to fetch weather data with new units");
                        // Revert to previous weather display
                        Self::update_weather_display(s, &state_manager_clone);
                    }))
                    .unwrap();
            }
        });
    }

    pub fn setup_auto_refresh(&self, siv: &mut Cursive) {
        let cb_sink = siv.cb_sink().clone();
        let state_manager_clone = self.state_manager.clone();

        thread::spawn(move || loop {
            thread::sleep(std::time::Duration::from_secs(AUTO_REFRESH_INTERVAL));

            if state_manager_clone.needs_refresh() {
                // Fetch new data when cache expires
                let current_location = state_manager_clone.get_current_location();
                let state_for_refresh = state_manager_clone.clone();

                let _ = cb_sink.send(Box::new(move |s| {
                    let fetcher = WeatherFetcher::new(state_for_refresh);
                    fetcher.switch_location(s, current_location);
                }));
            } else {
                // Update display to show current cache age without fetching new data
                let state_for_display = state_manager_clone.clone();
                let _ = cb_sink.send(Box::new(move |s| {
                    // Update cache_age in context to current time difference
                    state_for_display.update_cache_age();
                    Self::update_weather_display(s, &state_for_display);
                }));
            }
        });
    }

    fn update_ui_loading(&self, siv: &mut Cursive) {
        siv.call_on_name(WEATHER_HEADER_NAME, |view: &mut TextView| {
            view.set_content(WeatherDisplay::format_loading_message());
        });
        siv.call_on_name(WEATHER_CURRENT_NAME, |view: &mut TextView| {
            view.set_content(WeatherDisplay::format_wait_message());
        });
        siv.call_on_name(WEATHER_FORECAST_NAME, |view: &mut TextView| {
            view.set_content("");
        });
        siv.call_on_name(DATA_AGE_PROGRESS_NAME, |view: &mut ProgressBar| {
            view.set_value(0);
        });
    }

    fn update_weather_display(siv: &mut Cursive, state_manager: &TuiStateManager) {
        if state_manager.is_loading() {
            return;
        }

        let context = state_manager.get_context();
        let header_text = WeatherDisplay::format_header_text(&context);
        let current_info = WeatherDisplay::format_current_info(&context);
        let forecast_text = WeatherDisplay::format_forecast_text(&context);

        siv.call_on_name(WEATHER_HEADER_NAME, |view: &mut TextView| {
            view.set_content(header_text);
        });
        siv.call_on_name(WEATHER_CURRENT_NAME, |view: &mut TextView| {
            view.set_content(current_info);
        });
        siv.call_on_name(WEATHER_FORECAST_NAME, |view: &mut TextView| {
            view.set_content(forecast_text);
        });

        // Update data age progress bar
        let cache_duration = WEATHER_CACHE_DURATION;
        let progress_percentage =
            ((context.cache_age as f64 / cache_duration as f64) * 100.0).min(100.0) as usize;
        siv.call_on_name(DATA_AGE_PROGRESS_NAME, |view: &mut ProgressBar| {
            view.set_value(progress_percentage);
        });
    }

    fn show_error_dialog(siv: &mut Cursive, message: &str) {
        siv.add_layer(cursive::views::Dialog::text(message).title("Error").button("OK", |s| {
            s.pop_layer();
        }));
    }

    fn fetch_weather_for_location(
        location: &str,
        state_manager: &TuiStateManager,
    ) -> Result<Context, Box<dyn std::error::Error + Send + Sync>> {
        let mut settings = state_manager.get_settings();

        // Handle special "Automatic" case for IP-based lookup
        if location == "Automatic" {
            settings.location = String::new(); // Empty string triggers IP lookup
        } else {
            // Parse location for geocoding API
            let parts: Vec<&str> = location.split(',').collect();
            if parts.len() != 2 {
                return Err("Invalid location format".into());
            }
            settings.location = location.to_string();
        }

        // Fetch location data
        let location_data = LocationData::get_cached(settings.clone())?;

        // Fetch weather data
        let weather_data = Weather::get_cached(location_data.latitude, location_data.longitude, settings.clone())?;

        // Build context
        let context = Context::build(weather_data, location_data, settings);

        Ok(context)
    }
}

impl Clone for WeatherFetcher {
    fn clone(&self) -> Self {
        Self { state_manager: self.state_manager.clone() }
    }
}

impl Clone for TuiStateManager {
    fn clone(&self) -> Self {
        Self { state: self.state.clone() }
    }
}
