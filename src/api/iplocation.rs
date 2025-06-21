use crate::api::{Location, LocationData};
use isahc::prelude::*;
use url::Url;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IPLocation {
    pub city: String,
    pub country_code: String,
    pub lat: f64,
    pub lon: f64,
}

impl Location for IPLocation {
    /*
     * TODO:
     * Should check X-Rl and X-Ttl headers for rate limiting, but no easy
     * way to notify the user without a panic or error. Can we cache the response in a state file?
     */
    fn fetch(_name: &str, _country_code: &str) -> LocationData {
        let api_url = build_url("33603794");

        let mut response = isahc::get(api_url).expect("Failed to send Location request");
        if !response.status().is_success() {
            panic!("Failed to fetch location: {}", response.status());
        }
        let body = response.text().expect("Failed to read Location response body");
        let loc: IPLocation =
            serde_json::from_str(&body).expect("Failed to parse Location JSON response");

        LocationData {
            city: loc.city,
            country_code: loc.country_code,
            latitude: loc.lat,
            longitude: loc.lon,
        }
    }
}

fn build_url(field_code: &str) -> String {
    let base_url = "http://ip-api.com/json";
    let mut url = Url::parse(base_url).expect("Failed to parse base URL");

    url.query_pairs_mut().append_pair("fields", field_code);

    url.to_string()
}
