pub mod detailed;
pub mod json;
pub mod plain;
pub mod waybar;

use crate::api::LocationData;
use crate::weather::Weather;

pub trait Output {
    fn new(weather: Weather, location: LocationData) -> Self;
    fn render(&self) -> String;
}
