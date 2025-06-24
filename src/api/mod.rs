pub mod geolocation;
pub mod iplocation;
pub mod weather;

use crate::utils::*;
use crate::Settings;

use savefile::prelude::*;
use savefile_derive::Savefile;
use serde::{Deserialize, Serialize};

pub trait Location {
    fn fetch(name: &str, country_code: &str) -> LocationData;
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
    pub fn get_cached(s: Settings) -> Self {
        let filename = get_cached_file("location", &s.location, s.units);
        let now = get_now();

        let fd: LocationData = load_file(&filename, 0).unwrap_or_default();
        let l = s.location.to_owned();

        // Cache lifetime is 4 hours (14400 seconds)
        if fd.location == l && fd.created_at > 0 && now - fd.created_at < 14400 {
            if cfg!(debug_assertions) {
                println!("Using cached location data");
            }
            return fd;
        }

        let mut data = Self::lookup(l);
        data.latitude = format!("{:.1}", data.latitude).parse().unwrap_or(0.0);
        data.longitude = format!("{:.1}", data.longitude).parse().unwrap_or(0.0);

        match save_file(&filename, 0, &data) {
            Ok(_) => {
                if cfg!(debug_assertions) {
                    println!("Wrote location data to disk");
                }
            },
            Err(e) => eprintln!("Unable to save location data to disk: {:#?}", e),
        }

        data
    }

    fn lookup(l: String) -> Self {
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
