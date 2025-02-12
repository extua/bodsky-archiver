use anyhow::Result;
use reqwest::{
    header::{self},
    Client, Url,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{env, process};

// local libraries
use bodsky_archiver::call_api;

fn create_twitter_client() -> Result<Client> {
    const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
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

async fn collect_api_responses() -> Result<Vec<String>> {
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TweetFeed {
        data: Vec<Value>,
    }

    let twitter_client: Client = create_twitter_client().unwrap_or_else(|error| {
        // exit to stderr
        eprintln!("Failed to create twitter client: {error}");
        process::exit(1)
    });

    println!("{twitter_client:?}");

    let endpoint = Url::parse_with_params(
        "https://api.x.com/2/tweets/search/recent",
        &[
            ("query", "Oxford"),
            ("max_results", "10"),
            ("tweet.fields", "created_at,id,note_tweet"),
        ],
    )?;

    println!("calling this endpoint {endpoint:?}");

    let response: String = call_api(&twitter_client, &endpoint).await?;

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

pub async fn get_twitter_posts() {
    let response = collect_api_responses().await.unwrap_or_else(|error| {
        // exit to stderr
        eprintln!("Failed to collect tweets: {error}");
        process::exit(1)
    });

    println!("{response:?}");
}
