use anyhow::Result;
use reqwest::{header, Client};
use std::env;

struct TwitterClient(Client);

impl TwitterClient {
    fn new() -> Result<Client> {
        const APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
        let mut headers: header::HeaderMap = header::HeaderMap::new();
        // Todo: proper error handling here
        dotenvy::dotenv()?;
        let bearer_token: String = env::var("TWITTER_API_BEARER_TOKEN")?;
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(&bearer_token)?,
        );
        let app_client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .default_headers(headers)
            .gzip(true)
            .build()?;
        Ok(app_client)
    }
}

pub fn get_twitter_posts() {
    let twitter_client: Result<Client> = TwitterClient::new();
    println!("{:?}", twitter_client);
}
