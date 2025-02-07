use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use reqwest::{
    header::{self, RETRY_AFTER},
    Client, Error, Response, StatusCode, Url,
};
use serde::{de::Expected, Deserialize, Serialize};
use serde_json::Value;
use std::{env, process, time::Duration};
use tokio::time::sleep;
struct TwitterClient(Client);

impl TwitterClient {
    fn new() -> Result<Client> {
        const APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
        let mut headers: header::HeaderMap = header::HeaderMap::new();
        // Todo: proper error handling here
        dotenvy::dotenv()?;
        let bare_bearer_token: String = env::var("TWITTER_API_BEARER_TOKEN")?;
        let mut prefixed_bearer_token: String = "Bearer ".to_string();
        prefixed_bearer_token.push_str(&bare_bearer_token);

        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(&prefixed_bearer_token)?,
        );
        let app_client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .default_headers(headers)
            .gzip(true)
            .build()?;
        Ok(app_client)
    }
}

// todo, put this into lib.rs and use it for bluesky too
async fn request_tweets_from_api(
    app_client: Client,
    endpoint: Url,
) -> Result<String, reqwest::Error> {
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

async fn collect_api_responses() -> Result<Vec<String>> {
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TweetFeed {
        data: Vec<Value>,
    }

    let twitter_client: Client = match TwitterClient::new() {
        Ok(client) => client,
        Err(error) => panic!("Failed to create Twitter client: {error:?}"),
    };
    println!("{:?}", twitter_client);
    let endpoint = Url::parse_with_params(
        "https://api.x.com/2/tweets/search/recent",
        &[
            ("query", "Oxford"),
            ("max_results", "10"),
            ("tweet.fields", "created_at,id,note_tweet"),
        ],
    )
    .unwrap();

    println!("calling this endpoint {endpoint:?}");

    let response = request_tweets_from_api(twitter_client, endpoint).await?;

    // println!("{response:?}");

    // parse the response into tweet struct
    let bulk_posts: TweetFeed = serde_json::from_str(&response)?;

    let mut feed: Vec<String> = Vec::with_capacity(10);

    for post in bulk_posts.data {
        let id: &str = post["id"].as_str().unwrap();
        let text: &str = post["text"].as_str().unwrap();
        let created_at = post["created_at"].as_str().unwrap();
        let formatted_post = format!("\ntweet with id {id} posted at {created_at}\n{text}\n");
        print!("{formatted_post}");
        feed.push(formatted_post);
    }

    Ok(feed)
}

#[tokio::main]
pub async fn get_twitter_posts() {
    let response = collect_api_responses().await.unwrap_or_else(|error| {
        // exit to stderr
        eprintln!("Failed to collect tweets: {error}");
        process::exit(1)
    });

    println!("{:?}", response);
}
