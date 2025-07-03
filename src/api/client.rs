use anyhow::{Context, Result};
use isahc::config::Configurable;
use isahc::{HttpClient, HttpClientBuilder, ReadResponseExt};
use std::sync::OnceLock;
use std::time::Duration;

static HTTP_CLIENT: OnceLock<HttpClient> = OnceLock::new();

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
