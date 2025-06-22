pub mod geolocation;
pub mod iplocation;
pub mod weather;
use disk::*;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub trait Location {
    fn fetch(name: &str, country_code: &str) -> LocationData;
}

json!(LocationData, Dir::Data, env!("CARGO_PKG_NAME"), "location", "data");
#[derive(Default, Deserialize, Serialize, Debug)]
pub struct LocationData {
    pub city: String,
    pub country_code: String,
    pub latitude: f64,
    pub longitude: f64,
    pub location: String,
    pub created_at: u64,
}

impl LocationData {
    pub fn get_cached(l: String, use_cache: bool) -> Self {
        let now: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let fd = LocationData::from_file().unwrap_or_default();

        if use_cache && fd.location == l && fd.created_at > 0 && now - fd.created_at < 3600 {
            if cfg!(debug_assertions) {
                println!("Using cached location data: {:#?}", fd);
            }
            return fd;
        }

        let mut data = Self::lookup(l);
        data.latitude = format!("{:.1}", data.latitude).parse().unwrap_or(0.0);
        data.longitude = format!("{:.1}", data.longitude).parse().unwrap_or(0.0);

        match data.save_atomic() {
            Ok(_) => {
                if cfg!(debug_assertions) {
                    println!("Wrote location data to disk: {:#?}", data);
                }
            },
            Err(e) => eprintln!("Failed saving location data to disk: {:#?}", e),
        }

        data
    }

    pub fn lookup(l: String) -> Self {
        if !l.is_empty() {
            let parts: Vec<&str> = l.split(',').collect();
            if parts.len() == 2 {
                let name = parts[0].trim();
                let country_code = parts[1].trim().to_uppercase();
                geolocation::GeoLocation::fetch(name, &country_code)
            } else {
                panic!("Invalid location format. Use 'City, CountryCode'.");
            }
        } else {
            iplocation::IPLocation::fetch("", "")
        }
    }
}
