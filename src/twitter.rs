use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::{header, Client, Response, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{env, time::Duration};
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

fn collect_api_responses() -> Vec<String> {

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TweetFeed {
        data: Vec<Value>,
    }

    #[tokio::main]
    async fn request_tweets_from_api(app_client: Client) -> TweetFeed {
        let mut retries: u8 = 0;
        let mut backoff: Duration = Duration::from_secs(1);

        let response_from_retry: Result<Response, reqwest::Error> = loop {
            match app_client
                .get("https://api.x.com/2/tweets/search/recent")
                .query(&[("query", "Oxford"), ("max_results", "10"), ("tweet.fields", "created_at,id,text")])
                .send()
                .await
                // if status is 429, back off and retry
            {
                Ok(resp) if resp.status().is_success() => break Ok(resp),
                Ok(resp) if resp.status() == StatusCode::TOO_MANY_REQUESTS && retries < 6 => {
                    sleep(backoff).await;
                    retries += 1;
                    backoff *= 2;
                }
                Err(e) => break Err(e),
                // Breaking out with an error is fine,
                // the last match arm should never be met
                _ => panic!("Failed to request profile from API"),
            }
        };
        // first error handiing on the response
        let response: Response = match response_from_retry {
            Ok(response) => response,
            Err(network_error) => panic!("Failed to get API response: {network_error:?}"),
        };
        // parse the response into tweet struct
        let parsed_response: TweetFeed = match response.json::<TweetFeed>().await {
            Ok(response) => response,
            Err(parse_error) => panic!("Failed to parse API response: {parse_error:?}"),
        };
        parsed_response
    }

    let twitter_client: Client = match TwitterClient::new() {
        Ok(client) => client,
        Err(error) => panic!("Failed to create Twitter client: {error:?}"),
    };
    println!("{:?}", twitter_client);
    let bulk_posts = request_tweets_from_api(twitter_client);

    let mut feed: Vec<String> = Vec::with_capacity(10);

    for post in bulk_posts.data {
        let id: &str = post["id"].as_str().unwrap();
        let text: &str = post["text"].as_str().unwrap();
        let created_at = post["created_at"].as_str().unwrap();
        let formatted_post = format!("tweet with {id} posted at {created_at}\n{text}");
        println!("{formatted_post}");
        feed.push(formatted_post);
    }
    feed
}

pub fn get_twitter_posts() {
    let response: Vec<String> = collect_api_responses();
    println!("{:?}", response);
}
