pub mod geolocation;
pub mod iplocation;
pub mod weather;

pub trait Location {
    fn fetch(name: &str, country_code: &str) -> LocationData;
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocationData {
    pub city: String,
    pub country_code: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl LocationData {
    /*
     * TODO:
     * Store location data in a state file to reduce API lookups
     * Need to index it somehow so we can still look up by multiple city or country codes
     */
    pub fn lookup(location: String) -> Self {
        if !location.is_empty() {
            let parts: Vec<&str> = location.split(',').collect();
            if parts.len() == 2 {
                let name = parts[0].trim();
                let country_code = parts[1].trim().to_uppercase();
                geolocation::GeoLocation::fetch(name, &country_code)
            } else {
                eprintln!("Invalid location format. Use 'City, CountryCode'.");
                Self {
                    city: String::new(),
                    country_code: String::new(),
                    latitude: 0.0,
                    longitude: 0.0,
                }
            }
        } else {
            iplocation::IPLocation::fetch("", "")
        }
    }
}
