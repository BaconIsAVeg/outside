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
    fn fetch(n: &str, c: &str) -> Result<LocationData> {
        let base_url = "https://geocoding-api.open-meteo.com/v1/search";
        let params =
            vec![("name", n), ("countryCode", c), ("count", "1"), ("language", "en"), ("format", "json")];
        let api_url = utils::urls::builder(base_url, params);

        let body = client::get_with_retry(&api_url, 2)
            .with_context(|| format!("Unable to fetch location data for {}, {}", n, c))?;

        let loc: GeoLocation =
            serde_json::from_str(&body).with_context(|| "Failed to parse location response JSON")?;

        if loc.results.is_empty() {
            return Err(anyhow::anyhow!("No location results found for {}, {}", n, c));
        }

        let result = &loc.results[0];
        let city = result.name.to_owned();
        let country_code = result.country_code.to_owned();

        Ok(LocationData {
            city,
            country_code,
            latitude: result.latitude,
            longitude: result.longitude,
            location: format!("{}, {}", result.name, result.country_code),
            created_at: utils::get_now(),
        })
    }
}
