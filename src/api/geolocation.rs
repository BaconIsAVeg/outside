use crate::api::location::*;
use crate::utils;

use isahc::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeoLocation {
    pub results: Vec<Results>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Results {
    pub admin2: String,
    pub country_code: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl Location for GeoLocation {
    fn fetch(n: &str, c: &str) -> LocationData {
        let base_url = "https://geocoding-api.open-meteo.com/v1/search";
        let params = vec![("name", n), ("countryCode", c)];
        let api_url = utils::urls::builder(base_url, params);

        let mut response = isahc::get(api_url).expect("Unable to send Location request");
        if !response.status().is_success() {
            panic!("Unable to fetch location: {}", response.status());
        }
        let body = response.text().expect("Unable to read Location response body");
        let loc: GeoLocation = serde_json::from_str(&body).expect("Unable to parse Location JSON response");

        let city = loc.results[0].admin2.to_owned();
        let country_code = loc.results[0].country_code.to_owned();

        LocationData {
            city,
            country_code,
            latitude: loc.results[0].latitude,
            longitude: loc.results[0].longitude,
            location: format!("{}, {}", loc.results[0].admin2, loc.results[0].country_code),
            created_at: utils::get_now(),
        }
    }
}
