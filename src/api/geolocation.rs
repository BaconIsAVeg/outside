use crate::api::client;
use crate::api::location::*;
use crate::utils;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeoLocation {
    pub results: Vec<Results>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Results {
    pub name: String,
    pub country_code: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl Location for GeoLocation {
    /// Fetches location data using the Open-Meteo geocoding API.
    ///
    /// Searches for a location by name and country code, returning the first result.
    /// Uses the Open-Meteo geocoding API to find coordinates for the specified location.
    ///
    /// # Arguments
    ///
    /// * `n` - The city or location name to search for
    /// * `c` - The country code to narrow down the search
    ///
    /// # Returns
    ///
    /// Returns `LocationData` containing the location details and coordinates.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The API request fails
    /// - The JSON response cannot be parsed
    /// - No results are found for the specified location
    fn fetch(n: &str, c: &str) -> Result<LocationData> {
        let base_url = "https://geocoding-api.open-meteo.com/v1/search";
        let params =
            vec![("name", n), ("countryCode", c), ("count", "10"), ("language", "en"), ("format", "json")];
        let api_url = utils::urls::builder(base_url, params);

        let body = client::get_with_retry(&api_url, 2)
            .with_context(|| format!("Unable to fetch location data for {}, {}", n, c))?;

        let loc: GeoLocation =
            serde_json::from_str(&body).with_context(|| "Failed to parse location response JSON")?;

        if loc.results.is_empty() {
            return Err(anyhow::anyhow!("No location results found for {}, {}", n, c));
        }

        let result = &loc.results[0];

        let mut location_data = LocationData {
            city: result.name.to_owned(),
            country_code: result.country_code.to_owned(),
            latitude: result.latitude,
            longitude: result.longitude,
            location: format!("{}, {}", result.name, result.country_code),
            created_at: utils::get_now(),
        };

        // Normalize the location data for consistent formatting
        location_data.normalize();

        Ok(location_data)
    }
}
