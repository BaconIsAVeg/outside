use crate::context::Context;
use crate::settings::Units;
use crate::Settings;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct TuiState {
    pub context: Context,
    pub settings: Settings,
    pub loading: bool,
    pub last_fetch_time: u64,
    pub weather_created_at: u64,
}

pub struct TuiStateManager {
    pub state: Arc<Mutex<TuiState>>,
}

impl TuiStateManager {
    pub fn new(context: Context, settings: Settings) -> Self {
        let now = crate::utils::get_now();
        let weather_created_at = now - context.cache_age;
        let initial_state = TuiState {
            context: context.clone(),
            settings,
            loading: false,
            last_fetch_time: weather_created_at,
            weather_created_at,
        };
        let state = Arc::new(Mutex::new(initial_state));
        Self { state }
    }

    pub fn get_state(&self) -> Arc<Mutex<TuiState>> {
        self.state.clone()
    }

    pub fn set_loading(&self, loading: bool) {
        let mut state_guard = self.state.lock().unwrap();
        state_guard.loading = loading;
    }

    pub fn update_context(&self, context: Context) {
        let mut state_guard = self.state.lock().unwrap();
        let now = crate::utils::get_now();
        let weather_created_at = now - context.cache_age;

        state_guard.context = context;
        state_guard.loading = false;
        state_guard.last_fetch_time = now;
        state_guard.weather_created_at = weather_created_at;
    }

    pub fn get_current_location(&self) -> String {
        let state_guard = self.state.lock().unwrap();
        if state_guard.settings.location.is_empty() {
            "Automatic".to_string()
        } else {
            format!("{}, {}", state_guard.context.city, state_guard.context.country)
        }
    }

    pub fn toggle_units(&self) -> Units {
        let mut state_guard = self.state.lock().unwrap();
        state_guard.settings.units = match state_guard.settings.units {
            Units::Metric => Units::Imperial,
            Units::Imperial => Units::Metric,
        };
        state_guard.settings.units.clone()
    }

    pub fn get_settings(&self) -> Settings {
        let state_guard = self.state.lock().unwrap();
        state_guard.settings.clone()
    }

    pub fn needs_refresh(&self) -> bool {
        let state_guard = self.state.lock().unwrap();
        let now = crate::utils::get_now();
        now - state_guard.last_fetch_time > super::constants::WEATHER_CACHE_DURATION
    }

    pub fn is_loading(&self) -> bool {
        let state_guard = self.state.lock().unwrap();
        state_guard.loading
    }

    pub fn get_context(&self) -> Context {
        let state_guard = self.state.lock().unwrap();
        state_guard.context.clone()
    }

    pub fn update_cache_age(&self) {
        let mut state_guard = self.state.lock().unwrap();
        let now = crate::utils::get_now();
        state_guard.context.cache_age = now - state_guard.weather_created_at;
    }
}
