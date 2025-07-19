use crate::api::location::LocationData;
use crate::tui::async_operations::WeatherFetcher;
use crate::tui::constants::*;
use crate::tui::location_manager::LocationManager;
use crate::tui::state_manager::TuiStateManager;
use crate::tui::ui_components::UiComponents;
use cursive::views::{Dialog, EditView, SelectView};
use cursive::Cursive;

pub struct KeyboardHandlers;

impl KeyboardHandlers {
    pub fn setup_all_handlers(
        siv: &mut Cursive,
        state_manager: TuiStateManager,
        location_manager: LocationManager,
        weather_fetcher: WeatherFetcher,
    ) {
        Self::setup_quit_handlers(siv);
        Self::setup_location_handlers(siv, state_manager.clone(), location_manager, weather_fetcher.clone());
        Self::setup_unit_toggle_handler(siv, weather_fetcher);
        Self::setup_forecast_toggle_handler(siv, state_manager);
    }

    fn setup_quit_handlers(siv: &mut Cursive) {
        siv.add_global_callback('q', |s| s.quit());
        siv.add_global_callback(cursive::event::Key::Esc, |s| {
            // If there are dialog layers, close the top one; otherwise quit
            if s.screen().len() > 1 {
                s.pop_layer();
            } else {
                s.quit();
            }
        });
    }

    fn setup_location_handlers(
        siv: &mut Cursive,
        state_manager: TuiStateManager,
        location_manager: LocationManager,
        weather_fetcher: WeatherFetcher,
    ) {
        // Add location
        Self::setup_add_location_handler(
            siv,
            state_manager.clone(),
            location_manager.clone(),
            weather_fetcher.clone(),
        );

        // Delete location
        Self::setup_delete_location_handler(siv, location_manager.clone());

        // Switch location (Enter key)
        Self::setup_switch_location_handler(siv, weather_fetcher);
    }

    fn setup_add_location_handler(
        siv: &mut Cursive,
        state_manager: TuiStateManager,
        location_manager: LocationManager,
        weather_fetcher: WeatherFetcher,
    ) {
        siv.add_global_callback('a', move |s| {
            let state_manager_clone = state_manager.clone();
            let location_manager_clone = location_manager.clone();
            let weather_fetcher_clone = weather_fetcher.clone();

            let add_location_fn = {
                let state_manager_for_add = state_manager_clone.clone();
                let location_manager_for_add = location_manager_clone.clone();
                let weather_fetcher_for_add = weather_fetcher_clone.clone();

                move |s: &mut Cursive| {
                    let location = s
                        .call_on_name(NEW_LOCATION_NAME, |view: &mut EditView| view.get_content())
                        .unwrap()
                        .to_string();

                    if !location.is_empty() {
                        let normalized_location = if location != "Automatic" {
                            LocationData::normalize_location_string(&location)
                        } else {
                            location.clone()
                        };

                        // Check if normalized location already exists
                        let location_list = location_manager_for_add.get_location_list();
                        let list = location_list.lock().unwrap();
                        if list.locations.contains(&normalized_location) {
                            drop(list);
                            s.add_layer(
                                Dialog::text(format!(
                                    "Location '{normalized_location}' is already in the list."
                                ))
                                .title("Bookmark Already Exists")
                                .button("OK", |s| {
                                    s.pop_layer();
                                }),
                            );
                            return;
                        }
                        drop(list);

                        // Add and switch to the new location
                        Self::add_and_switch_location(
                            s,
                            &state_manager_for_add,
                            location_manager_for_add.clone(),
                            &weather_fetcher_for_add,
                            normalized_location,
                        );
                    }
                    s.pop_layer();
                }
            };

            let add_fn_clone = add_location_fn.clone();

            // Create dialog with submit handler for Enter key
            let dialog = UiComponents::create_add_location_dialog(move |s, _content| {
                add_fn_clone(s);
            })
            .button("Add", add_location_fn)
            .button("Cancel", |s| {
                s.pop_layer();
            });

            s.add_layer(dialog);
        });
    }

    fn setup_delete_location_handler(siv: &mut Cursive, location_manager: LocationManager) {
        siv.add_global_callback('d', move |s| {
            let (selected_index, selected_location) = s
                .call_on_name(LOCATION_LIST_NAME, |view: &mut SelectView<String>| {
                    let index = view.selected_id();
                    let location = view.selection().map(|sel| sel.as_ref().clone());
                    (index, location)
                })
                .unwrap_or((None, None));

            if let (Some(index), Some(location)) = (selected_index, selected_location) {
                let location_manager_clone = location_manager.clone();
                let dialog = UiComponents::create_delete_confirmation_dialog(&location)
                    .button("Delete", move |s| {
                        location_manager_clone.remove_location_by_name(&location);

                        // Update the select view
                        s.call_on_name(LOCATION_LIST_NAME, |view: &mut SelectView<String>| {
                            view.remove_item(index);
                        });
                        s.pop_layer();
                    })
                    .button("Cancel", |s| {
                        s.pop_layer();
                    });

                s.add_layer(dialog);
            }
        });
    }

    fn setup_switch_location_handler(siv: &mut Cursive, weather_fetcher: WeatherFetcher) {
        siv.add_global_callback(cursive::event::Key::Enter, move |s| {
            let selected = s.call_on_name(LOCATION_LIST_NAME, |view: &mut SelectView<String>| {
                view.selection().map(|sel| sel.as_ref().clone())
            });

            if let Some(Some(location)) = selected {
                weather_fetcher.switch_location(s, location);
            }
        });
    }

    fn setup_unit_toggle_handler(siv: &mut Cursive, weather_fetcher: WeatherFetcher) {
        siv.add_global_callback('u', move |s| {
            weather_fetcher.toggle_units(s);
        });
    }

    fn setup_forecast_toggle_handler(siv: &mut Cursive, state_manager: TuiStateManager) {
        siv.add_global_callback('f', move |s| {
            state_manager.toggle_forecast_mode();
            UiComponents::update_weather_display_components(s, &state_manager);
        });
    }

    fn add_and_switch_location(
        siv: &mut Cursive,
        _state_manager: &TuiStateManager,
        location_manager: LocationManager,
        weather_fetcher: &WeatherFetcher,
        location: String,
    ) {
        let location_clone = location.clone();
        let location_manager_clone = location_manager.clone();

        weather_fetcher.fetch_and_update(
            location,
            siv,
            move |s, state_manager, result| {
                // Update both context and currently selected location
                state_manager.update_context_with_location(result, location_clone.clone());

                // Add to location list
                location_manager_clone.add_location(location_clone.clone());

                // Update the select view and select the new location
                s.call_on_name(LOCATION_LIST_NAME, |view: &mut SelectView<String>| {
                    let target_index = location_manager_clone.rebuild_select_view(view, &location_clone);
                    if let Some(index) = target_index {
                        view.set_selection(index);
                    }
                });

                // Update the weather display
                UiComponents::update_weather_display_components(s, state_manager);
            },
            |s, state_manager, error_message| {
                // Show error dialog and revert to previous weather display
                Self::show_error_dialog(s, &error_message);
                UiComponents::update_weather_display_components(s, state_manager);
            },
        );
    }

    fn show_error_dialog(siv: &mut Cursive, message: &str) {
        siv.add_layer(Dialog::text(message).title("Error").button("OK", |s| {
            s.pop_layer();
        }));
    }
}
