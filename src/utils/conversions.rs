use chrono::{NaiveDate, NaiveDateTime};

/// Converts an ISO8601 datetime string to a human-readable time format.
///
/// Takes a datetime string in the format "YYYY-MM-DDTHH:MM" and converts it
/// to a 12-hour time format with AM/PM indicator (e.g., "08:30am").
///
/// # Arguments
///
/// * `iso8601` - A datetime string in ISO8601 format
///
/// # Returns
///
/// Returns a formatted time string in 12-hour format with lowercase AM/PM.
///
/// # Panics
///
/// Panics if the input string cannot be parsed as a valid ISO8601 datetime.
pub fn iso8601_to_time(iso8601: String) -> String {
    let dt = NaiveDateTime::parse_from_str(&iso8601, "%Y-%m-%dT%H:%M").unwrap();
    dt.format("%I:%M%P").to_string()
}

/// Converts an ISO8601 date string to a human-readable date format.
///
/// Takes a date string in the format "YYYY-MM-DD" and converts it to
/// a readable format showing the day of week and month/day (e.g., "Mon 03/15").
///
/// # Arguments
///
/// * `iso8601` - A date string in ISO8601 format
///
/// # Returns
///
/// Returns a formatted date string with abbreviated day name and MM/DD format.
///
/// # Panics
///
/// Panics if the input string cannot be parsed as a valid ISO8601 date.
pub fn iso8601_to_date(iso8601: String) -> String {
    let dt = NaiveDate::parse_from_str(&iso8601, "%Y-%m-%d").unwrap();
    dt.format("%a %m/%d").to_string()
}
