use crate::api::{Location, LocationData};
use isahc::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IPLocation {
    pub city: String,
    pub country_code: String,
    pub lat: f64,
    pub lon: f64,
}

impl Location for IPLocation {
    fn fetch(_: &str, _: &str) -> LocationData {
        let api_url = build_url("33603794");

        let mut response = isahc::get(api_url).expect("Unable to request location data");
        if !response.status().is_success() {
            panic!("Unable to fetch location data: {}", response.status());
        }
        let body = response.text().expect("Unable to read Location response body");
        let loc: IPLocation = serde_json::from_str(&body).expect("Unable to parse Location JSON response");

        LocationData {
            city: loc.city.to_owned(),
            country_code: loc.country_code.to_owned(),
            latitude: loc.lat,
            longitude: loc.lon,
            location: "".to_string(),
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        }
    }
}

fn build_url(field_code: &str) -> String {
    let base_url = "http://ip-api.com/json";
    let mut url = Url::parse(base_url).expect("Unable to parse base location URL");

    url.query_pairs_mut().append_pair("fields", field_code);

    url.to_string()
}
