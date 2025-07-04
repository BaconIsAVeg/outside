pub mod async_operations;
pub mod constants;
pub mod keyboard_handlers;
pub mod location_manager;
pub mod state_manager;
pub mod ui_components;
pub mod weather_display;

use crate::context::Context;
use crate::output::Output;
use crate::Settings;
use async_operations::WeatherFetcher;
use cursive::views::ResizedView;
use cursive::{Cursive, CursiveExt};
use keyboard_handlers::KeyboardHandlers;
use location_manager::LocationManager;
use state_manager::TuiStateManager;
use ui_components::UiComponents;

#[derive(Debug)]
pub struct TuiOutput {
    context: Context,
    settings: Settings,
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
        UiComponents::setup_theme(&mut siv);

        // Initialize managers
        let state_manager = TuiStateManager::new(self.context.clone(), self.settings.clone());
        let location_manager = LocationManager::new();
        let weather_fetcher = WeatherFetcher::new(state_manager.clone());

        // Add current location to list if not present
        let current_location = location_manager.get_current_location_string(&self.settings.location);
        location_manager.ensure_location_in_list(current_location);

        // Create main layout
        let main_layout = UiComponents::create_main_layout(&state_manager, &location_manager, &self.settings);
        siv.add_fullscreen_layer(ResizedView::with_full_screen(main_layout));

        // Set initial progress bar value based on current cache age
        let context = state_manager.get_context();
        let initial_progress = ((context.cache_age as f64 / constants::WEATHER_CACHE_DURATION as f64) * 100.0)
            .min(100.0) as usize;
        siv.call_on_name(constants::DATA_AGE_PROGRESS_NAME, |view: &mut cursive::views::ProgressBar| {
            view.set_value(initial_progress);
        });

        // Set up keyboard handlers
        KeyboardHandlers::setup_all_handlers(
            &mut siv,
            state_manager.clone(),
            location_manager,
            weather_fetcher.clone(),
        );

        // Set up automatic refresh when cache expires
        weather_fetcher.setup_auto_refresh(&mut siv);

        // Run the TUI
        siv.run();
    }
}
