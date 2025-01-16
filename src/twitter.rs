use reqwest::{header, Client};
use std::{env, fmt::Error};

struct TwitterClient(Client);

impl TwitterClient {
    fn new() -> Client {
        const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
    let mut headers = header::HeaderMap::new();
    // Todo: proper error handling here
    dotenvy::dotenv().expect("Something wrong with dotenvy");
    let bearer_token: String =
        env::var("TWITTER_API_BEARER_TOKEN").expect("Unable to read the bearer token from .env");
    headers.insert(
        "Authorization",
        header::HeaderValue::from_str(&bearer_token)
            .expect("Unable to insert bearer token into HeaderValue"),
    );
    let app_client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .default_headers(headers)
        .build()
        .expect("unable to create client");
    app_client
    }
}

pub fn get_twitter_posts() {
    let twitter_client: Client = TwitterClient::new();

}
