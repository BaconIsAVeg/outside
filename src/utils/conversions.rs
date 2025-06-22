use chrono::NaiveDateTime;

pub fn iso8601_to_time(iso8601: String) -> String {
    let dt = NaiveDateTime::parse_from_str(&iso8601, "%Y-%m-%dT%H:%M").unwrap();
    dt.format("%I:%M%P").to_string()
}
