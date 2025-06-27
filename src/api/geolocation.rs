use crate::api::location::*;
use crate::utils::*;

use isahc::prelude::*;
use serde::{Deserialize, Serialize};
use url::Url;

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
        let api_url = build_url(n, c);

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
            created_at: get_now(),
        }
    }
}

// TODO: Refactor this into a common function
fn build_url(n: &str, c: &str) -> String {
    let base_url = "https://geocoding-api.open-meteo.com/v1/search";
    let mut url = Url::parse(base_url).expect("Unable to parse base URL");

    url.query_pairs_mut()
        .append_pair("name", n)
        .append_pair("countryCode", c)
        .append_pair("count", "1")
        .append_pair("language", "en")
        .append_pair("format", "json");

    url.to_string()
}
