use reqwest::{header, Client};
use std::env;

struct TwitterClient(Client);

#[derive(Debug)]
pub enum TwitterClientError {
    ReadEnvFile(dotenvy::Error),
    General(String)
}

impl std::fmt::Display for TwitterClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TwitterClientError::ReadEnvFile(e) => write!(f, "Failure to read env file: {}", e),
            TwitterClientError::General(s) => write!(f, "General error: {}", s),
        }
    }
}
impl std::error::Error for TwitterClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TwitterClientError::ReadEnvFile(e) => Some(e),
            TwitterClientError::General(_) => None
        }
    }
}

impl From<dotenvy::Error> for TwitterClientError {
    fn from(cause: dotenvy::Error) -> TwitterClientError {
        TwitterClientError::ReadEnvFile(cause)
    }
}

impl TwitterClient {
    fn new() -> Result<Client, TwitterClientError> {
        const APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
        let mut headers: header::HeaderMap = header::HeaderMap::new();
        // Todo: proper error handling here
        dotenvy::dotenv()?;
        let bearer_token: String = env::var("TWITTER_API_BEARER_TOKEN")
            .expect("Unable to read the bearer token from .env");
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
        Ok(app_client)
    }
}

pub fn get_twitter_posts() {
    let twitter_client: Result<Client, TwitterClientError> = TwitterClient::new();
    println!("{:?}", twitter_client);
}
