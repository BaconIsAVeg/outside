use crate::api::client;
use crate::api::location::*;
use crate::utils;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IPLocation {
    pub city: String,
    pub country_code: String,
    pub lat: f64,
    pub lon: f64,
}

impl Location for IPLocation {
    /// Fetches location data based on the client's IP address.
    ///
    /// Uses the ip-api.com service to determine the user's location based on their
    /// public IP address. This is used when no explicit location is provided.
    ///
    /// # Arguments
    ///
    /// * `_` - Unused parameter (name), kept for trait compatibility
    /// * `_` - Unused parameter (country_code), kept for trait compatibility
    ///
    /// # Returns
    ///
    /// Returns `LocationData` containing the detected location and coordinates.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The API request fails
    /// - The JSON response cannot be parsed
    /// - Network connectivity issues prevent IP detection
    fn fetch(_: &str, _: &str) -> Result<LocationData> {
        let base_url = "http://ip-api.com/json";
        let api_url = utils::urls::builder(base_url, vec![("fields", "33603794")]);

        let body =
            client::get_with_retry(&api_url, 2).with_context(|| "Unable to fetch IP-based location data")?;

        let loc: IPLocation =
            serde_json::from_str(&body).with_context(|| "Unable to parse IP location response JSON")?;

        let mut location_data = LocationData {
            city: loc.city.to_owned(),
            country_code: loc.country_code.to_owned(),
            latitude: loc.lat,
            longitude: loc.lon,
            location: "".to_string(),
            created_at: utils::get_now(),
        };

        // Normalize the location data for consistent formatting
        location_data.normalize();

        Ok(location_data)
    }
}
