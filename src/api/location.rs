use crate::utils::*;
use crate::Settings;

use crate::api::geolocation;
use crate::api::iplocation;

use anyhow::Result;
use savefile::prelude::*;
use savefile_derive::Savefile;
use serde::{Deserialize, Serialize};
use stringcase;

/// Trait for different location lookup methods.
///
/// This trait abstracts the location lookup functionality, allowing for different
/// implementations such as city-based geocoding or IP-based location detection.
pub trait Location {
    /// Fetches location data for the specified name and country code.
    ///
    /// # Arguments
    ///
    /// * `name` - The city or location name to look up
    /// * `country_code` - The country code (may be empty for IP-based lookup)
    ///
    /// # Returns
    ///
    /// Returns `LocationData` containing coordinates and location information.
    ///
    /// # Errors
    ///
    /// Returns an error if the location cannot be found or API request fails.
    fn fetch(name: &str, country_code: &str) -> Result<LocationData>;
}

#[derive(Default, Deserialize, Serialize, Debug, Savefile)]
pub struct LocationData {
    pub city: String,
    pub country_code: String,
    pub latitude: f64,
    pub longitude: f64,
    pub location: String,
    pub created_at: u64,
}

impl LocationData {
    /// Normalizes a city name to Title Case format using stringcase.
    ///
    /// This helper function converts city names to consistent Title Case format:
    /// - "new york" -> "New York"
    /// - "los angeles" -> "Los Angeles"
    /// - "san francisco" -> "San Francisco"
    ///
    /// # Arguments
    ///
    /// * `city` - The city name to normalize
    ///
    /// # Returns
    ///
    /// The normalized city name in Title Case format
    pub fn normalize_city_name(city: &str) -> String {
        // Convert to lowercase first, then capitalize each word using stringcase
        // Since stringcase doesn't have title_case, we'll manually split and capitalize
        city.split_whitespace().map(stringcase::pascal_case).collect::<Vec<_>>().join(" ")
    }

    /// Normalizes a location string to consistent "City, COUNTRY" format.
    ///
    /// This helper function takes a location string and returns it in normalized format:
    /// - City names are converted to CamelCase
    /// - Country codes are converted to uppercase
    ///
    /// # Arguments
    ///
    /// * `location` - The location string to normalize (e.g., "new york, us")
    ///
    /// # Returns
    ///
    /// The normalized location string (e.g., "New York, US")
    pub fn normalize_location_string(location: &str) -> String {
        if location.is_empty() {
            return location.to_string();
        }

        let parts: Vec<&str> = location.split(',').collect();
        if parts.len() == 2 {
            let city = parts[0].trim();
            let country = parts[1].trim().to_uppercase();
            let normalized_city = Self::normalize_city_name(city);
            format!("{}, {}", normalized_city, country)
        } else {
            location.to_string()
        }
    }

    /// Normalizes the location data by formatting city as CamelCase and country as uppercase.
    ///
    /// This method ensures consistent formatting across all location data:
    /// - City names are converted to CamelCase (e.g., "new york" -> "New York")
    /// - Country codes are converted to uppercase (e.g., "us" -> "US")
    /// - The location string is updated to reflect the normalized format
    ///
    /// # Examples
    ///
    /// ```
    /// let mut location = LocationData {
    ///     city: "new york".to_string(),
    ///     country_code: "us".to_string(),
    ///     location: "new york, us".to_string(),
    ///     ..Default::default()
    /// };
    /// location.normalize();
    /// assert_eq!(location.city, "New York");
    /// assert_eq!(location.country_code, "US");
    /// assert_eq!(location.location, "New York, US");
    /// ```
    pub fn normalize(&mut self) {
        // Convert city to CamelCase
        self.city = Self::normalize_city_name(&self.city);

        // Convert country code to uppercase
        self.country_code = self.country_code.to_uppercase();

        // Update location string with normalized format
        if !self.city.is_empty() && !self.country_code.is_empty() {
            self.location = format!("{}, {}", self.city, self.country_code);
        }
    }

    /// Retrieves location data using cached data if available.
    ///
    /// Location data is cached for 4 hours (14400 seconds) to reduce API calls.
    /// If cached data is found for the same location and is still fresh, it will be returned.
    /// Otherwise, fresh data will be fetched using the appropriate lookup method.
    ///
    /// # Arguments
    ///
    /// * `s` - Settings containing location string and units for cache key generation
    ///
    /// # Returns
    ///
    /// Returns location data on success, or an error if lookup fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The location format is invalid (for manual location entry)
    /// - The API request fails
    /// - No location results are found
    pub fn get_cached(s: Settings) -> Result<Self> {
        let filename = cache::get_cached_file("location", &s.location, s.units);
        let now = get_now();

        let fd: LocationData = load_file(&filename, 0).unwrap_or_default();
        let l = s.location.to_owned();

        // Normalize the input location for cache comparison
        let normalized_input = Self::normalize_location_string(&l);

        // Cache lifetime is 4 hours (14400 seconds)
        if fd.location == normalized_input && fd.created_at > 0 && now - fd.created_at < 14400 {
            return Ok(fd);
        }

        let mut data = Self::lookup(l)?;
        data.latitude = format!("{:.1}", data.latitude).parse().unwrap_or(0.0);
        data.longitude = format!("{:.1}", data.longitude).parse().unwrap_or(0.0);

        match save_file(&filename, 0, &data) {
            Ok(_) => {},
            Err(e) => eprintln!("Unable to save location data to disk: {:#?}", e),
        }

        Ok(data)
    }

    /// Looks up location data based on the provided location string.
    ///
    /// If the location string is empty, uses IP-based location detection.
    /// If the location string contains a comma, treats it as "City, CountryCode" format
    /// and uses geocoding API.
    ///
    /// # Arguments
    ///
    /// * `l` - Location string, either empty (for IP lookup) or "City, CountryCode" format
    ///
    /// # Returns
    ///
    /// Returns location data on success, or an error if the lookup fails.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The location format is invalid (not "City, CountryCode")
    /// - The geocoding or IP location API request fails
    /// - No results are found for the specified location
    fn lookup(l: String) -> Result<Self> {
        if !l.is_empty() {
            let parts: Vec<&str> = l.split(',').collect();
            if parts.len() == 2 {
                let name = parts[0].trim();
                let country_code = parts[1].trim().to_uppercase();

                // Normalize the city name to CamelCase for consistency
                let normalized_name = Self::normalize_city_name(name);

                geolocation::GeoLocation::fetch(&normalized_name, &country_code)
            } else {
                Err(anyhow::anyhow!("Invalid location format. Use 'City, CountryCode'."))
            }
        } else {
            iplocation::IPLocation::fetch("", "")
        }
    }
}
