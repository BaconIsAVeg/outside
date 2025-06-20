use crate::api::{Location, LocationData};
use isahc::prelude::*;
use url::Url;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
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
    fn fetch(name: &str, country_code: &str) -> LocationData {
        let api_url = build_url(name, country_code);

        let mut response = isahc::get(api_url).expect("Failed to send Location request");
        if !response.status().is_success() {
            panic!("Failed to fetch location: {}", response.status());
        }
        let body = response.text().expect("Failed to read Location response body");
        let loc: GeoLocation =
            serde_json::from_str(&body).expect("Failed to parse Location JSON response");

        LocationData {
            city: loc.results[0].admin2.to_owned(),
            country_code: loc.results[0].country_code.to_owned(),
            latitude: loc.results[0].latitude,
            longitude: loc.results[0].longitude,
        }
    }
}

fn build_url(name: &str, country_code: &str) -> String {
    let base_url = "https://geocoding-api.open-meteo.com/v1/search";
    let mut url = Url::parse(base_url).expect("Failed to parse base URL");

    url.query_pairs_mut()
        .append_pair("name", name)
        .append_pair("countryCode", country_code)
        .append_pair("count", "1")
        .append_pair("language", "en")
        .append_pair("format", "json");

    url.to_string()
}
