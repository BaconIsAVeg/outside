use crate::api::location::LocationData;
use crate::api::weather::Weather;
use crate::context::Context;
use crate::output::Output;
use crate::utils::cache;
use crate::Settings;
use cursive::align::HAlign;
use cursive::theme::{Color, PaletteColor, Theme};
use cursive::view::{Nameable, Resizable};
use cursive::views::{Dialog, EditView, LinearLayout, Panel, ResizedView, SelectView, TextView};
use cursive::{Cursive, CursiveExt};
use savefile::prelude::*;
use savefile_derive::Savefile;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Serialize, Deserialize, Debug, Default, Savefile)]
pub struct LocationList {
    pub locations: Vec<String>,
}

impl LocationList {
    fn load() -> Self {
        let filename = cache::get_cached_file("locations", "list", crate::settings::Units::Metric);
        load_file(&filename, 0).unwrap_or_default()
    }

    fn save(&self) {
        let filename = cache::get_cached_file("locations", "list", crate::settings::Units::Metric);
        if let Err(e) = save_file(&filename, 0, self) {
            eprintln!("Unable to save location list: {:#?}", e);
        }
    }

    fn add_location(&mut self, location: String) {
        if !self.locations.contains(&location) {
            self.locations.push(location);
            self.save();
        }
    }

    fn remove_location_by_name(&mut self, location: &str) {
        if let Some(index) = self.locations.iter().position(|loc| loc == location) {
            self.locations.remove(index);
            self.save();
        }
    }
}

#[derive(Debug)]
pub struct TuiOutput {
    context: Context,
    settings: Settings,
}

#[derive(Debug, Clone)]
struct TuiState {
    context: Context,
    settings: Settings,
    loading: bool,
    last_fetch_time: u64,
}

impl Output for TuiOutput {
    /// Creates a new TuiOutput instance.
    ///
    /// The TUI output mode creates an interactive terminal interface
    /// displaying weather information in a structured layout.
    ///
    /// # Arguments
    ///
    /// * `context` - Weather and location data to be displayed
    /// * `settings` - Settings parameter for location management
    ///
    /// # Returns
    ///
    /// Returns a TuiOutput instance with the provided context.
    fn new(context: Context, settings: Settings) -> Self {
        TuiOutput { context, settings }
    }

    /// Renders the TUI interface and returns empty string.
    ///
    /// The TUI mode doesn't return text output but instead displays
    /// an interactive terminal interface. This method launches the
    /// cursive application and blocks until the user exits.
    ///
    /// # Returns
    ///
    /// Returns an empty string since output is handled by the TUI.
    fn render(&self) -> String {
        self.run_tui();
        String::new()
    }
}

impl TuiOutput {
    /// Runs the interactive TUI interface.
    ///
    /// Creates a full-screen cursive application with weather information
    /// displayed across the entire terminal with location management on the right.
    /// Users can navigate using keyboard controls and exit by pressing 'q' or Escape.
    fn run_tui(&self) {
        let mut siv = Cursive::default();

        // Set up theme with terminal default background and no shadows
        let mut theme = Theme::default();
        theme.palette[PaletteColor::Background] = Color::TerminalDefault;
        theme.palette[PaletteColor::View] = Color::TerminalDefault;
        theme.palette[PaletteColor::Primary] = Color::TerminalDefault;
        theme.palette[PaletteColor::Secondary] = Color::TerminalDefault;
        theme.palette[PaletteColor::Tertiary] = Color::TerminalDefault;
        theme.palette[PaletteColor::TitlePrimary] = Color::Dark(cursive::theme::BaseColor::Blue);
        theme.palette[PaletteColor::TitleSecondary] = Color::TerminalDefault;
        theme.palette[PaletteColor::Highlight] = Color::Dark(cursive::theme::BaseColor::Blue);
        theme.palette[PaletteColor::HighlightInactive] = Color::Dark(cursive::theme::BaseColor::Blue);
        theme.palette[PaletteColor::Shadow] = Color::TerminalDefault;
        theme.palette[PaletteColor::HighlightText] = Color::TerminalDefault;
        theme.shadow = false;
        siv.set_theme(theme);

        // Initialize shared state
        let now = crate::utils::get_now();
        let initial_state = TuiState {
            context: self.context.clone(),
            settings: self.settings.clone(),
            loading: false,
            last_fetch_time: now - self.context.cache_age, // Calculate when data was originally fetched
        };
        let state = Arc::new(Mutex::new(initial_state));

        // Load location list
        let location_list = Arc::new(Mutex::new(LocationList::load()));

        // Add current location to list if not present
        // If no location was specified in settings, add a "Automatic" entry for IP-based lookup
        let current_location = if self.settings.location.is_empty() {
            "Automatic".to_string()
        } else {
            format!("{}, {}", self.context.city, self.context.country)
        };
        {
            let mut list = location_list.lock().unwrap();
            list.add_location(current_location.clone());
        }

        // Create main layout
        self.create_tui_layout(&mut siv, &state, &location_list);

        // Set up global keybindings
        self.setup_keybindings(&mut siv, state.clone(), location_list);

        // Set up automatic refresh when cache expires
        self.setup_auto_refresh(&mut siv, state);

        // Run the TUI
        siv.run();
    }

    fn create_tui_layout(
        &self,
        siv: &mut Cursive,
        state: &Arc<Mutex<TuiState>>,
        location_list: &Arc<Mutex<LocationList>>,
    ) {
        // Create main weather display
        let weather_layout = self.create_weather_layout_from_state(state);

        // Create location list (full height)
        let location_select = self.create_location_list(location_list);

        // Create help bar (single line, full width)
        let help_text = "Enter: Select bookmark  |  a: Add bookmark  |  d: Delete bookmark  |  u: Toggle units  |  q/Esc: Quit";
        let help_bar = TextView::new(help_text).h_align(HAlign::Center);

        // Create main content area with weather on left and locations on right
        let main_content = LinearLayout::horizontal()
            .child(ResizedView::with_full_width(weather_layout))
            .child(ResizedView::with_fixed_width(30, location_select));

        // Create full layout with main content and bottom help bar
        let full_layout =
            LinearLayout::vertical().child(ResizedView::with_full_height(main_content)).child(help_bar);

        // Use the full screen
        siv.add_fullscreen_layer(ResizedView::with_full_screen(full_layout));
    }

    fn create_weather_layout_from_state(&self, state: &Arc<Mutex<TuiState>>) -> LinearLayout {
        let state_guard = state.lock().unwrap();
        let context = &state_guard.context;

        if state_guard.loading {
            return LinearLayout::vertical().child(
                Panel::new(TextView::new("Loading weather data...").h_align(HAlign::Left))
                    .title("Weather")
                    .title_position(cursive::align::HAlign::Left),
            );
        }

        let header_text = Self::format_header_text(context);
        let current_info = Self::format_current_info(context);
        let forecast_text = Self::format_forecast_text(context);

        LinearLayout::vertical()
            .child(
                Panel::new(TextView::new(header_text).h_align(HAlign::Center).with_name("weather_header"))
                    .title(format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")))
                    .title_position(cursive::align::HAlign::Left),
            )
            .child(ResizedView::with_full_height(
                Panel::new(TextView::new(current_info).with_name("weather_current")).title("Current"),
            ))
            .child(
                Panel::new(TextView::new(forecast_text).with_name("weather_forecast"))
                    .title("7 Day Forecast"),
            )
    }

    fn create_location_list(
        &self,
        location_list: &Arc<Mutex<LocationList>>,
    ) -> Panel<cursive::views::NamedView<SelectView<String>>> {
        let mut select = SelectView::new();

        // Ensure CLI location is in the location list for selection
        if !self.settings.location.is_empty() {
            let normalized_cli_location = LocationData::normalize_location_string(&self.settings.location);
            let mut list = location_list.lock().unwrap();
            if !list.locations.contains(&normalized_cli_location) {
                list.add_location(normalized_cli_location);
            }
        }

        // Populate the list with sorted locations
        let list = location_list.lock().unwrap();
        let sorted_locations = list.locations.clone();

        // Separate "Automatic" from other locations
        let mut automatic_locations = Vec::new();
        let mut other_locations = Vec::new();

        for location in sorted_locations {
            if location == "Automatic" {
                automatic_locations.push(location);
            } else {
                other_locations.push(location);
            }
        }

        // Sort other locations by city, then country code
        other_locations.sort_by(|a, b| {
            let a_parts: Vec<&str> = a.split(',').collect();
            let b_parts: Vec<&str> = b.split(',').collect();

            if a_parts.len() >= 2 && b_parts.len() >= 2 {
                let a_city = a_parts[0].trim();
                let a_country = a_parts[1].trim();
                let b_city = b_parts[0].trim();
                let b_country = b_parts[1].trim();

                // Sort by city first, then by country
                a_city.cmp(b_city).then(a_country.cmp(b_country))
            } else {
                // Fallback to string comparison for malformed entries
                a.cmp(b)
            }
        });

        // Create the ordered list for both SelectView and finding the correct selection index
        let mut all_ordered_locations = automatic_locations.clone();
        all_ordered_locations.extend(other_locations.clone());

        // Add items to select view in the same order
        for location in &all_ordered_locations {
            select.add_item(location.clone(), location.clone());
        }

        // Set current location as selected
        let current_location = if self.settings.location.is_empty() {
            "Automatic".to_string()
        } else {
            // Use normalized location string from CLI argument to match the location list
            LocationData::normalize_location_string(&self.settings.location)
        };

        // Find the correct index in the SelectView order
        if let Some(index) = all_ordered_locations.iter().position(|loc| *loc == current_location) {
            select.set_selection(index);
        }

        Panel::new(select.with_name("location_list")).title("Bookmarks")
    }

    fn setup_keybindings(
        &self,
        siv: &mut Cursive,
        state: Arc<Mutex<TuiState>>,
        location_list: Arc<Mutex<LocationList>>,
    ) {
        // Quit commands
        siv.add_global_callback('q', |s| s.quit());
        siv.add_global_callback(cursive::event::Key::Esc, |s| {
            // If there are dialog layers, close the top one; otherwise quit
            if s.screen().len() > 1 {
                s.pop_layer();
            } else {
                s.quit();
            }
        });

        // Add location
        let list_clone = location_list.clone();
        let state_clone = state.clone();
        siv.add_global_callback('a', move |s| {
            let add_location_fn = {
                let list = list_clone.clone();
                let state_for_add = state_clone.clone();
                move |s: &mut Cursive| {
                    let location = s
                        .call_on_name("new_location", |view: &mut EditView| view.get_content())
                        .unwrap()
                        .to_string();

                    if !location.is_empty() {
                        // Normalize the location string: CamelCase city and uppercase country code
                        let normalized_location = if location != "Automatic" {
                            LocationData::normalize_location_string(&location)
                        } else {
                            location.clone()
                        };

                        // Check if normalized location already exists
                        {
                            let list = list.lock().unwrap();
                            if list.locations.contains(&normalized_location) {
                                drop(list); // Release the lock before showing dialog
                                s.add_layer(
                                    Dialog::text(format!(
                                        "Location '{}' is already in the list.",
                                        normalized_location
                                    ))
                                    .title("Bookmark Already Exists")
                                    .button("OK", |s| {
                                        s.pop_layer();
                                    }),
                                );
                                return;
                            }
                        }

                        // Try to add and switch to the new location
                        Self::add_and_switch_location(s, &state_for_add, &list, normalized_location);
                    }
                    s.pop_layer();
                }
            };

            let add_fn_clone = add_location_fn.clone();
            let edit_view = EditView::new()
                .content("")
                .on_submit(move |s, _| {
                    add_fn_clone(s);
                })
                .with_name("new_location")
                .fixed_width(30);

            s.add_layer(
                Dialog::around(edit_view)
                    .title("Add Location (City, Country)")
                    .button("Add", add_location_fn)
                    .button("Cancel", |s| {
                        s.pop_layer();
                    }),
            );
        });

        // Delete location
        let list_clone = location_list.clone();
        siv.add_global_callback('d', move |s| {
            let (selected_index, selected_location) = s
                .call_on_name("location_list", |view: &mut SelectView<String>| {
                    let index = view.selected_id();
                    let location = view.selection().map(|sel| sel.as_ref().clone());
                    (index, location)
                })
                .unwrap_or((None, None));

            if let (Some(index), Some(location)) = (selected_index, selected_location) {
                let list_for_dialog = list_clone.clone();
                s.add_layer(
                    Dialog::text(format!("Are you sure you want to delete '{}'?", location))
                        .title("Confirm Deletion")
                        .button("Delete", move |s| {
                            let mut list = list_for_dialog.lock().unwrap();
                            list.remove_location_by_name(&location);

                            // Update the select view
                            s.call_on_name("location_list", |view: &mut SelectView<String>| {
                                view.remove_item(index);
                            });
                            s.pop_layer();
                        })
                        .button("Cancel", |s| {
                            s.pop_layer();
                        }),
                );
            }
        });

        // Switch location (Enter key)
        let state_clone = state.clone();
        siv.add_global_callback(cursive::event::Key::Enter, move |s| {
            let selected = s.call_on_name("location_list", |view: &mut SelectView<String>| {
                view.selection().map(|sel| sel.as_ref().clone())
            });

            if let Some(Some(location)) = selected {
                Self::switch_location(s, &state_clone, location);
            }
        });

        // Toggle units (u key)
        let state_clone = state.clone();
        siv.add_global_callback('u', move |s| {
            Self::toggle_units(s, &state_clone);
        });
    }

    fn switch_location(siv: &mut Cursive, state: &Arc<Mutex<TuiState>>, location: String) {
        // Set loading state
        {
            let mut state_guard = state.lock().unwrap();
            state_guard.loading = true;
        }

        // Update display to show loading
        let state_clone = state.clone();
        siv.call_on_name("weather_header", |view: &mut TextView| {
            view.set_content("Loading weather data...");
        });
        siv.call_on_name("weather_current", |view: &mut TextView| {
            view.set_content("Please wait...");
        });
        siv.call_on_name("weather_forecast", |view: &mut TextView| {
            view.set_content("");
        });

        // Get a handle to the cursive instance for async update
        let cb_sink = siv.cb_sink().clone();
        let location_clone = location.clone();

        // Spawn background thread to fetch weather data
        thread::spawn(move || {
            if let Ok(result) = Self::fetch_weather_for_location(&location_clone, &state_clone) {
                // Send callback to update UI on main thread
                cb_sink
                    .send(Box::new(move |s| {
                        // Update state with new data
                        {
                            let mut state_guard = state_clone.lock().unwrap();
                            state_guard.context = result;
                            state_guard.loading = false;
                            state_guard.last_fetch_time = crate::utils::get_now();
                        }

                        // Update the display
                        Self::update_display_from_state(s, &state_clone);
                    }))
                    .unwrap();
            } else {
                // Handle error case
                cb_sink
                    .send(Box::new(move |s| {
                        {
                            let mut state_guard = state_clone.lock().unwrap();
                            state_guard.loading = false;
                        }
                        s.add_layer(
                            Dialog::text(format!("Failed to fetch weather data for: {}", location_clone))
                                .title("Error")
                                .button("OK", |s| {
                                    s.pop_layer();
                                }),
                        );
                    }))
                    .unwrap();
            }
        });
    }

    fn toggle_units(siv: &mut Cursive, state: &Arc<Mutex<TuiState>>) {
        // Get current location for re-fetching weather data
        let current_location = {
            let state_guard = state.lock().unwrap();
            format!("{}, {}", state_guard.context.city, state_guard.context.country)
        };

        // Toggle units in state
        {
            let mut state_guard = state.lock().unwrap();
            state_guard.settings.units = match state_guard.settings.units {
                crate::settings::Units::Metric => crate::settings::Units::Imperial,
                crate::settings::Units::Imperial => crate::settings::Units::Metric,
            };
            state_guard.loading = true;
        }

        // Update display to show loading
        siv.call_on_name("weather_header", |view: &mut TextView| {
            view.set_content("Switching units...");
        });
        siv.call_on_name("weather_current", |view: &mut TextView| {
            view.set_content("Please wait...");
        });
        siv.call_on_name("weather_forecast", |view: &mut TextView| {
            view.set_content("");
        });

        // Get a handle to the cursive instance for async update
        let cb_sink = siv.cb_sink().clone();
        let state_clone = state.clone();

        // Spawn background thread to fetch weather data with new units
        thread::spawn(move || {
            if let Ok(result) = Self::fetch_weather_for_location(&current_location, &state_clone) {
                // Send callback to update UI on main thread
                cb_sink
                    .send(Box::new(move |s| {
                        // Update state with new data
                        {
                            let mut state_guard = state_clone.lock().unwrap();
                            state_guard.context = result;
                            state_guard.loading = false;
                            state_guard.last_fetch_time = crate::utils::get_now();
                        }

                        // Update the display
                        Self::update_display_from_state(s, &state_clone);
                    }))
                    .unwrap();
            } else {
                // Handle error case
                cb_sink
                    .send(Box::new(move |s| {
                        {
                            let mut state_guard = state_clone.lock().unwrap();
                            state_guard.loading = false;
                        }
                        s.add_layer(
                            Dialog::text("Failed to fetch weather data with new units")
                                .title("Error")
                                .button("OK", |s| {
                                    s.pop_layer();
                                }),
                        );
                    }))
                    .unwrap();
            }
        });
    }

    fn add_and_switch_location(
        siv: &mut Cursive,
        state: &Arc<Mutex<TuiState>>,
        location_list: &Arc<Mutex<LocationList>>,
        location: String,
    ) {
        // Normalize the location string: uppercase country code
        let normalized_location = if location != "Automatic" {
            let parts: Vec<&str> = location.split(',').collect();
            if parts.len() == 2 {
                let city = parts[0].trim();
                let country = parts[1].trim().to_uppercase();
                format!("{}, {}", city, country)
            } else {
                location.clone()
            }
        } else {
            location.clone()
        };
        // Set loading state
        {
            let mut state_guard = state.lock().unwrap();
            state_guard.loading = true;
        }

        // Update display to show loading
        let state_clone = state.clone();
        siv.call_on_name("weather_header", |view: &mut TextView| {
            view.set_content("Loading weather data...");
        });
        siv.call_on_name("weather_current", |view: &mut TextView| {
            view.set_content("Please wait...");
        });
        siv.call_on_name("weather_forecast", |view: &mut TextView| {
            view.set_content("");
        });

        // Get a handle to the cursive instance for async update
        let cb_sink = siv.cb_sink().clone();
        let location_clone = normalized_location.clone();
        let location_list_clone = location_list.clone();

        // Spawn background thread to fetch weather data
        thread::spawn(move || {
            if let Ok(result) = Self::fetch_weather_for_location(&location_clone, &state_clone) {
                // Success: Add location to list and UI, then update display
                cb_sink
                    .send(Box::new(move |s| {
                        // Add to location list
                        {
                            let mut list = location_list_clone.lock().unwrap();
                            list.add_location(location_clone.clone());
                        }

                        // Update the select view and select the new location
                        s.call_on_name("location_list", |view: &mut SelectView<String>| {
                            let new_index = view.len();
                            view.add_item(location_clone.clone(), location_clone.clone());
                            view.set_selection(new_index);
                        });

                        // Update state with new data
                        {
                            let mut state_guard = state_clone.lock().unwrap();
                            state_guard.context = result;
                            state_guard.loading = false;
                            state_guard.last_fetch_time = crate::utils::get_now();
                        }

                        // Update the display
                        Self::update_display_from_state(s, &state_clone);
                    }))
                    .unwrap();
            } else {
                // Failure: Show error and don't add location to list
                cb_sink
                    .send(Box::new(move |s| {
                        {
                            let mut state_guard = state_clone.lock().unwrap();
                            state_guard.loading = false;
                        }
                        s.add_layer(
                            Dialog::text(format!("Failed to fetch weather data for: {}\nThis location may be invalid and was not added.", location_clone))
                                .title("Invalid Location")
                                .button("OK", |s| {
                                    s.pop_layer();
                                }),
                        );

                        // Reset display to previous state
                        Self::update_display_from_state(s, &state_clone);
                    }))
                    .unwrap();
            }
        });
    }

    fn setup_auto_refresh(&self, siv: &mut Cursive, state: Arc<Mutex<TuiState>>) {
        let cb_sink = siv.cb_sink().clone();
        let state_clone = state.clone();

        thread::spawn(move || {
            loop {
                // Check every 30 seconds if weather data needs refreshing
                thread::sleep(std::time::Duration::from_secs(30));

                let needs_refresh = {
                    let state_guard = state_clone.lock().unwrap();
                    let now = crate::utils::get_now();
                    // Weather cache expires after 10 minutes (600 seconds)
                    now - state_guard.last_fetch_time > 600
                };

                if needs_refresh {
                    let current_location = {
                        let state_guard = state_clone.lock().unwrap();
                        if state_guard.settings.location.is_empty() {
                            "Automatic".to_string()
                        } else {
                            format!("{}, {}", state_guard.context.city, state_guard.context.country)
                        }
                    };

                    // Refresh the weather data
                    let state_for_refresh = state_clone.clone();
                    let _ = cb_sink.send(Box::new(move |s| {
                        Self::switch_location(s, &state_for_refresh, current_location);
                    }));
                }
            }
        });
    }

    fn fetch_weather_for_location(
        location: &str,
        state: &Arc<Mutex<TuiState>>,
    ) -> Result<Context, Box<dyn std::error::Error + Send + Sync>> {
        // Get current settings
        let settings = {
            let state_guard = state.lock().unwrap();
            let mut s = state_guard.settings.clone();

            // Handle special "Automatic" case for IP-based lookup
            if location == "Automatic" {
                s.location = String::new(); // Empty string triggers IP lookup
            } else {
                // Parse location for geocoding API
                let parts: Vec<&str> = location.split(',').collect();
                if parts.len() != 2 {
                    return Err("Invalid location format".into());
                }
                s.location = location.to_string();
            }
            s
        };

        // Fetch location data
        let location_data = LocationData::get_cached(settings.clone())?;

        // Fetch weather data
        let weather_data = Weather::get_cached(location_data.latitude, location_data.longitude, settings)?;

        // Build context
        let context = Context::build(weather_data, location_data);

        Ok(context)
    }

    fn update_display_from_state(siv: &mut Cursive, state: &Arc<Mutex<TuiState>>) {
        let state_guard = state.lock().unwrap();
        let context = &state_guard.context;

        if state_guard.loading {
            return;
        }

        let header_text = Self::format_header_text(context);
        let current_info = Self::format_current_info(context);
        let forecast_text = Self::format_forecast_text(context);

        // Apply updates to the views
        siv.call_on_name("weather_header", |view: &mut TextView| {
            view.set_content(header_text);
        });
        siv.call_on_name("weather_current", |view: &mut TextView| {
            view.set_content(current_info);
        });
        siv.call_on_name("weather_forecast", |view: &mut TextView| {
            view.set_content(forecast_text);
        });
    }

    fn format_header_text(context: &Context) -> String {
        format!(
            "{}, {}\n\
            {} {}{} • {} • Feels like {}{}",
            context.city,
            context.country,
            context.weather_icon,
            context.temperature.round(),
            context.temperature_unit,
            context.weather_description,
            context.feels_like.round(),
            context.temperature_unit
        )
    }

    fn format_current_info(context: &Context) -> String {
        format!(
            "Temperature:     {}{}\n\
            Humidity:        {}%\n\
            Pressure:        {} hPa\n\
            Wind:            {} {} with gusts up to {} {} ({})\n\
            UV Index:        {}\n\
            Precipitation:   {} {} ({}% chance)\n\
            Sun:             {} • {}",
            context.temperature.round(),
            context.temperature_unit,
            context.humidity,
            context.pressure,
            context.wind_speed.round(),
            context.wind_speed_unit,
            context.wind_gusts.round(),
            context.wind_speed_unit,
            context.wind_compass,
            context.uv_index,
            context.precipitation_sum,
            context.precipitation_unit,
            context.precipitation_chance,
            context.sunrise,
            context.sunset
        )
    }

    fn format_forecast_text(context: &Context) -> String {
        let mut forecast_text = String::new();
        for (index, day) in context.forecast.iter().enumerate() {
            let display_date = if index == 0 {
                "Today".to_string()
            } else if index == 1 {
                "Tomorrow".to_string()
            } else {
                day.date.clone()
            };
            forecast_text.push_str(&format!(
                "{:10} {} {:>2}-{:<2}{} {}\n",
                display_date,
                day.weather_icon,
                day.temperature_low.round(),
                day.temperature_high.round(),
                context.temperature_unit,
                day.weather_description
            ));
        }
        forecast_text
    }
}
