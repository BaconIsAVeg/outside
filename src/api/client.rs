use anyhow::{Context, Result};
use isahc::config::Configurable;
use isahc::{HttpClient, HttpClientBuilder, ReadResponseExt};
use std::sync::OnceLock;
use std::time::Duration;

static HTTP_CLIENT: OnceLock<HttpClient> = OnceLock::new();

/// Returns a shared HTTP client instance configured with appropriate timeouts and connection pooling.
///
/// The client is created once and reused for all HTTP requests to improve performance.
/// Configuration includes:
/// - 2 second connection timeout
/// - 5 second request timeout
/// - 15 second TCP keepalive
/// - Maximum 4 connections per host
///
/// # Returns
///
/// Returns a reference to the global HTTP client instance.
pub fn get_client() -> &'static HttpClient {
    HTTP_CLIENT.get_or_init(|| {
        HttpClientBuilder::new()
            .connect_timeout(Duration::from_secs(2))
            .timeout(Duration::from_secs(5))
            .tcp_keepalive(Duration::from_secs(15))
            .max_connections_per_host(4)
            .build()
            .expect("Unable to create HTTP client")
    })
}

/// Performs a GET request to the specified URL and returns the response body as a string.
///
/// # Arguments
///
/// * `url` - The URL to send the GET request to
///
/// # Returns
///
/// Returns the response body as a string on success, or an error if the request fails
/// or the response status is not successful.
///
/// # Errors
///
/// This function will return an error if:
/// - The HTTP request fails to send
/// - The response status indicates failure
/// - The response body cannot be read as text
pub fn get(url: &str) -> Result<String> {
    let client = get_client();

    let mut response = client.get(url).with_context(|| format!("Unable to send request to {}", url))?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "HTTP request failed with status: {} for URL: {}",
            response.status(),
            url
        ));
    }

    response.text().with_context(|| format!("Unable to read response body from {}", url))
}

/// Performs a GET request with exponential backoff retry logic.
///
/// Attempts the request up to `max_retries + 1` times (initial attempt plus retries).
/// Uses exponential backoff with a base delay of 100ms, doubling on each retry.
///
/// # Arguments
///
/// * `url` - The URL to send the GET request to
/// * `max_retries` - Maximum number of retry attempts after the initial request
///
/// # Returns
///
/// Returns the response body as a string on success, or the last error encountered
/// if all attempts fail.
///
/// # Errors
///
/// This function will return an error if all retry attempts fail. The error
/// returned will be from the final attempt.
pub fn get_with_retry(url: &str, max_retries: usize) -> Result<String> {
    let mut last_error = None;

    for attempt in 0..=max_retries {
        match get(url) {
            Ok(response) => return Ok(response),
            Err(e) => {
                last_error = Some(e);
                if attempt < max_retries {
                    // Simple backoff strategy: wait 100ms * 2^attempt
                    let delay = Duration::from_millis(100 * (2_u64.pow(attempt as u32)));
                    std::thread::sleep(delay);
                }
            },
        }
    }

    Err(last_error.unwrap())
}
