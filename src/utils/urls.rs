use url::Url;

/// Builds a URL with query parameters from a base URL and parameter list.
///
/// Takes a base URL and a vector of key-value pairs, then constructs a complete
/// URL with properly encoded query parameters. This is used throughout the
/// application for building API requests with multiple parameters.
///
/// # Arguments
///
/// * `base_url` - The base URL string to build upon
/// * `params` - Vector of (key, value) tuples to add as query parameters
///
/// # Returns
///
/// Returns a complete URL string with query parameters appended.
///
/// # Panics
///
/// Panics if the base URL cannot be parsed as a valid URL.
pub fn builder(base_url: &str, params: Vec<(&str, &str)>) -> String {
    let mut url = Url::parse(base_url).expect("Unable to parse base URL");

    url.query_pairs_mut().clear();
    for (key, value) in params {
        url.query_pairs_mut().append_pair(key, value);
    }

    url.to_string()
}
