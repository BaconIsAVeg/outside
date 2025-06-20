pub struct Units {
    pub temperature: String,
    pub wind_speed: String,
    pub precipitation: String,
}

impl Units {
    pub fn metric() -> Self {
        Units {
            temperature: String::from("celsius"),
            wind_speed: String::from("kmh"),
            precipitation: String::from("mm"),
        }
    }

    pub fn imperial() -> Self {
        Units {
            temperature: String::from("fahrenheit"),
            wind_speed: String::from("mph"),
            precipitation: String::from("inch"),
        }
    }
}
