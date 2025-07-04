use crate::api::location::LocationData;
use crate::tui::constants::*;
use crate::tui::location_manager::LocationManager;
use crate::tui::state_manager::TuiStateManager;
use crate::tui::weather_display::WeatherDisplay;
use crate::Settings;
use cursive::align::HAlign;
use cursive::theme::{Color, ColorType, PaletteColor, Theme};
use cursive::view::{Nameable, Resizable};
use cursive::views::{DummyView, LinearLayout, Panel, ProgressBar, ResizedView, SelectView, TextView};
use cursive::Cursive;

pub struct UiComponents;

impl UiComponents {
    pub fn setup_theme(siv: &mut Cursive) {
        let mut theme = Theme::default();
        theme.palette[PaletteColor::Background] = Color::TerminalDefault;
        theme.palette[PaletteColor::View] = Color::TerminalDefault;
        theme.palette[PaletteColor::Primary] = Color::TerminalDefault;
        theme.palette[PaletteColor::Secondary] = Color::TerminalDefault;
        theme.palette[PaletteColor::Tertiary] = Color::TerminalDefault;
        theme.palette[PaletteColor::TitlePrimary] = Color::Dark(cursive::theme::BaseColor::Blue);
        theme.palette[PaletteColor::TitleSecondary] = Color::TerminalDefault;
        theme.palette[PaletteColor::Highlight] = Color::Dark(cursive::theme::BaseColor::Blue);
        theme.palette[PaletteColor::HighlightInactive] = Color::Rgb(30, 30, 40);
        theme.palette[PaletteColor::Shadow] = Color::TerminalDefault;
        theme.palette[PaletteColor::HighlightText] = Color::TerminalDefault;
        theme.shadow = false;
        siv.set_theme(theme);
    }

    pub fn create_main_layout(
        state_manager: &TuiStateManager,
        location_manager: &LocationManager,
        settings: &Settings,
    ) -> LinearLayout {
        let weather_layout = Self::create_weather_layout(state_manager);
        let location_select = Self::create_location_panel(location_manager, settings);
        let help_bar = Self::create_help_bar();

        let main_content = LinearLayout::horizontal()
            .child(ResizedView::with_full_width(weather_layout))
            .child(ResizedView::with_fixed_width(LOCATION_LIST_WIDTH, location_select));

        LinearLayout::vertical().child(ResizedView::with_full_height(main_content)).child(help_bar)
    }

    pub fn create_weather_layout(state_manager: &TuiStateManager) -> LinearLayout {
        if state_manager.is_loading() {
            return LinearLayout::vertical().child(
                Panel::new(TextView::new(WeatherDisplay::format_loading_message()).h_align(HAlign::Left))
                    .title("Weather")
                    .title_position(cursive::align::HAlign::Left),
            );
        }

        let context = state_manager.get_context();
        let header_text = WeatherDisplay::format_header_text(&context);
        let current_info = WeatherDisplay::format_current_info(&context);
        let forecast_text = WeatherDisplay::format_forecast_text(&context);

        LinearLayout::vertical()
            .child(
                Panel::new(TextView::new(header_text).h_align(HAlign::Center).with_name(WEATHER_HEADER_NAME))
                    .title(format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")))
                    .title_position(cursive::align::HAlign::Left),
            )
            .child(ResizedView::with_full_height(
                Panel::new(
                    LinearLayout::vertical()
                        .child(TextView::new(current_info).with_name(WEATHER_CURRENT_NAME))
                        .child(DummyView.full_height())
                        .child(
                            ProgressBar::new()
                                .max(100)
                                .with_color(ColorType::Palette(PaletteColor::HighlightInactive))
                                .with_name(DATA_AGE_PROGRESS_NAME),
                        ),
                )
                .title("Current"),
            ))
            .child(
                Panel::new(TextView::new(forecast_text).with_name(WEATHER_FORECAST_NAME))
                    .title("7 Day Forecast"),
            )
    }

    pub fn create_location_panel(
        location_manager: &LocationManager,
        settings: &Settings,
    ) -> Panel<cursive::views::NamedView<SelectView<String>>> {
        let mut select = SelectView::new();

        // Ensure CLI location is in the location list for selection
        if !settings.location.is_empty() {
            let normalized_cli_location = LocationData::normalize_location_string(&settings.location);
            location_manager.ensure_location_in_list(normalized_cli_location);
        }

        // Get sorted locations
        let location_list = location_manager.get_location_list();
        let list = location_list.lock().unwrap();
        let (all_ordered_locations, _) = list.get_sorted_locations();
        drop(list);

        // Add items to select view
        for location in &all_ordered_locations {
            select.add_item(location.clone(), location.clone());
        }

        // Set current location as selected
        let current_location = location_manager.get_current_location_string(&settings.location);
        if let Some(index) = all_ordered_locations.iter().position(|loc| *loc == current_location) {
            select.set_selection(index);
        }

        Panel::new(select.with_name(LOCATION_LIST_NAME)).title("Bookmarks")
    }

    pub fn create_help_bar() -> TextView {
        let help_text = "Enter: Select bookmark  |  a: Add bookmark  |  d: Delete bookmark  |  u: Toggle units  |  q/Esc: Quit";
        TextView::new(help_text).h_align(HAlign::Center)
    }

    pub fn create_add_location_dialog<F>(on_submit: F) -> cursive::views::Dialog
    where
        F: Fn(&mut cursive::Cursive, &str) + 'static + Send + Sync,
    {
        let edit_view = cursive::views::EditView::new()
            .content("")
            .on_submit(on_submit)
            .with_name(NEW_LOCATION_NAME)
            .fixed_width(30);

        cursive::views::Dialog::around(edit_view).title("Add Location (City, Country)")
    }

    pub fn create_delete_confirmation_dialog(location: &str) -> cursive::views::Dialog {
        cursive::views::Dialog::text(format!("Are you sure you want to delete '{}'?", location))
            .title("Confirm Deletion")
    }

    pub fn update_weather_display_components(siv: &mut Cursive, state_manager: &TuiStateManager) {
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
    }
}
