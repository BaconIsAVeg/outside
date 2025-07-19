use crate::api::location::LocationData;
use crate::utils::cache;
use cursive::views::SelectView;
use savefile::prelude::*;
use savefile_derive::Savefile;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug, Default, Savefile)]
pub struct LocationList {
    pub locations: Vec<String>,
}

impl LocationList {
    pub fn load() -> Self {
        let filename = cache::get_cached_file("locations", "list");
        load_file(&filename, 0).unwrap_or_default()
    }

    pub fn save(&self) {
        let filename = cache::get_cached_file("locations", "list");
        if let Err(e) = save_file(&filename, 0, self) {
            eprintln!("Unable to save location list: {e:#?}");
        }
    }

    pub fn add_location(&mut self, location: String) {
        if !self.locations.contains(&location) {
            self.locations.push(location);
            self.save();
        }
    }

    pub fn remove_location_by_name(&mut self, location: &str) {
        if let Some(index) = self.locations.iter().position(|loc| loc == location) {
            self.locations.remove(index);
            self.save();
        }
    }

    /// Returns a sorted list of locations and the index of the specified location
    pub fn get_sorted_locations_with_index(&self, target_location: &str) -> (Vec<String>, Option<usize>) {
        let (sorted_locations, _) = self.get_sorted_locations();
        let target_index = sorted_locations.iter().position(|loc| loc == target_location);
        (sorted_locations, target_index)
    }

    /// Returns sorted locations with "Automatic" first, then alphabetically by city/country
    pub fn get_sorted_locations(&self) -> (Vec<String>, Vec<String>) {
        // Separate "Automatic" from other locations
        let mut automatic_locations = Vec::new();
        let mut other_locations = Vec::new();

        for location in &self.locations {
            if location == "Automatic" {
                automatic_locations.push(location.clone());
            } else {
                other_locations.push(location.clone());
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

        // Create the ordered list
        let mut all_ordered_locations = automatic_locations.clone();
        all_ordered_locations.extend(other_locations.clone());

        (all_ordered_locations, other_locations)
    }
}

#[derive(Clone)]
pub struct LocationManager {
    location_list: Arc<Mutex<LocationList>>,
}

impl Default for LocationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl LocationManager {
    pub fn new() -> Self {
        let location_list = Arc::new(Mutex::new(LocationList::load()));
        Self { location_list }
    }

    pub fn get_location_list(&self) -> Arc<Mutex<LocationList>> {
        self.location_list.clone()
    }

    pub fn add_location(&self, location: String) {
        let mut list = self.location_list.lock().unwrap();
        list.add_location(location);
    }

    pub fn remove_location_by_name(&self, location: &str) {
        let mut list = self.location_list.lock().unwrap();
        list.remove_location_by_name(location);
    }

    pub fn rebuild_select_view(&self, view: &mut SelectView<String>, target_location: &str) -> Option<usize> {
        let list = self.location_list.lock().unwrap();
        let (sorted_locations, target_index) = list.get_sorted_locations_with_index(target_location);

        // Clear and rebuild the SelectView with sorted locations
        view.clear();
        for location in &sorted_locations {
            view.add_item(location.clone(), location.clone());
        }

        target_index
    }

    pub fn get_current_location_string(&self, settings_location: &str) -> String {
        if settings_location.is_empty() {
            "Automatic".to_string()
        } else {
            LocationData::normalize_location_string(settings_location)
        }
    }

    pub fn ensure_location_in_list(&self, location: String) {
        let mut list = self.location_list.lock().unwrap();
        if !list.locations.contains(&location) {
            list.add_location(location);
        }
    }
}
