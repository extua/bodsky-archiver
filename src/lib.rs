use anyhow::Result;
use reqwest::{header::RETRY_AFTER, Client, Error, Response, StatusCode, Url};
use std::time::Duration;
use tokio::time::sleep;

pub async fn call_api(app_client: &Client, endpoint: &Url) -> Result<String, reqwest::Error> {
    let mut retries: u8 = 0;
    let mut backoff: Duration = Duration::from_secs(1);

    let endpoint_str: &str = endpoint.as_str();

    let response_from_retry: Result<Response, Error> = loop {
        match app_client
            .get(endpoint_str)
            .send()
            .await
            // if status is 429, back off and retry
        {
            Ok(resp) if resp.status().is_success() => break Ok(resp),
            Ok(resp) if resp.status() == StatusCode::TOO_MANY_REQUESTS && retries < 6 => {
                println!("Got a 429 error, sleeping {backoff:?} seconds");
                sleep(backoff).await;
                retries += 1;
                backoff *= 2;
            }
            // get the retry-after header value, convert it
            // to seconds, then to duration, etc.
            Ok(resp) if resp.headers().contains_key("retry-after") && retries < 6 => {
                if let Some(retry_after) = resp.headers().get(RETRY_AFTER) {
                    if let Ok(retry_after) = retry_after.to_str() {
                        if let Ok(retry_after) = retry_after.parse::<u64>() {
                            if retry_after < 128 {
                                backoff = Duration::from_secs(retry_after + 1);
                            }
                        }
                    }
                }
                println!("Got a retry-after response, sleeping {backoff:?} seconds");
                sleep(backoff).await;
                retries += 1;
                backoff *= 2;
            }
            Err(e) => break Err(e),
            // Breaking out with an error is fine,
            // the last match arm should never be met
            _ => panic!("Network request failed"),
        }
    };

    response_from_retry?.text().await
}
