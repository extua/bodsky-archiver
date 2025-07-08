use anyhow::Result;
use reqwest::{Client, Error, Response, StatusCode, Url, header::RETRY_AFTER};
use std::time::Duration;
// tokio sleep is non-blocking so yields the thread while waiting
use tokio::time::sleep;

pub async fn call_api(app_client: &Client, endpoint: &Url) -> Result<String, reqwest::Error> {
    const RETRY_SCALE: [u64; 12] = [1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233];

    let mut retries: usize = 0;

    let endpoint_str: &str = endpoint.as_str();

    let response_from_retry: Result<Response, Error> = loop {
        match app_client.get(endpoint_str).send().await {
            // if response is successful, return the response!
            Ok(resp) if resp.status().is_success() => break Ok(resp),
            // if status is 429, back off and retry
            Ok(resp)
                if resp.status() == StatusCode::TOO_MANY_REQUESTS
                    && retries < RETRY_SCALE.len() =>
            {
                // move along the retry scale, set new backoff duration, and sleep
                if let Some(backoff_value) = RETRY_SCALE.into_iter().nth(retries) {
                    let backoff: Duration = Duration::from_secs(backoff_value);
                    println!("Got a 429 error, sleeping {backoff:?} seconds");
                    sleep(backoff).await;
                }
                retries += 1;
            }
            // get the retry-after header value, convert it
            // to seconds, then to duration, etc.
            Ok(resp)
                if resp.headers().contains_key("retry-after") && retries < RETRY_SCALE.len() =>
            {
                if let Some(retry_after) = resp.headers().get(RETRY_AFTER)
                    && let Ok(retry_after) = retry_after.to_str()
                    && let Ok(retry_after) = retry_after.parse::<u64>()
                    && retry_after < 233
                {
                    let backoff = Duration::from_secs(retry_after + 1);
                    println!("Got a retry-after response, sleeping {backoff:?} seconds");
                    sleep(backoff).await;
                }
                retries += 1;
            }
            Err(e) => break Err(e),
            // Breaking out with an error is fine,
            // the last match arm should never be met
            _ => panic!("Network request failed"),
        }
    };

    response_from_retry?.text().await
}
