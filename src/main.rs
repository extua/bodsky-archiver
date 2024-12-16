use bodsky_archiver::convert_at_uri_to_url;
use core::panic;
use serde::{Deserialize, Serialize};
use serde_json::Value;
mod config;

fn get_posts_number() -> usize {
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Profile {
        posts_count: usize,
    }

    #[tokio::main]
    async fn request_profile_from_api() -> Result<Profile, reqwest::Error> {
        let raw_response: Result<Profile, reqwest::Error> = reqwest::Client::new()
            .get("https://public.api.bsky.app/xrpc/app.bsky.actor.getProfile")
            .query(&[("actor", config::ACCOUNT_DID)])
            .send()
            .await?
            .json::<Profile>()
            .await;
        raw_response
    }
    let response: Profile = match request_profile_from_api() {
        Ok(response) => response,
        Err(error) => panic!("Failed to get or parse API response: {error:?}"),
    };
    response.posts_count
}

fn collect_api_responses(total_posts: usize) {
    // This loop tracks the number of posts remaining and the number
    // to make in each api call
    let api_calls_needed: usize = total_posts.div_euclid(config::POSTS_PER_REQUEST) + 1;
    let mut current_call: usize = 1;
    let mut posts_remaining: usize = total_posts;
    let mut posts_to_request = config::POSTS_PER_REQUEST;
    while current_call <= api_calls_needed {
        if posts_remaining < config::POSTS_PER_REQUEST {
            posts_to_request = posts_remaining
        }
        println!("posts to request {}", posts_to_request);
        current_call += 1;
        posts_remaining -= posts_to_request;
    }

    #[tokio::main]
    async fn request_bulk_posts_from_api() -> Vec<Value> {
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct AuthorFeed {
            feed: Vec<Value>,
        }

        let posts_per_request_str: String = config::POSTS_PER_REQUEST.to_string();

        let raw_response: Result<AuthorFeed, reqwest::Error> = reqwest::Client::new()
            .get("https://public.api.bsky.app/xrpc/app.bsky.feed.getAuthorFeed")
            .query(&[
                ("actor", config::ACCOUNT_DID),
                ("limit", &posts_per_request_str),
            ])
            .send()
            .await
            .unwrap()
            .json::<AuthorFeed>()
            .await;
        let response: AuthorFeed = match raw_response {
            Ok(response) => response,
            Err(error) => panic!("Failed to get or parse API response: {error:?}"),
        };
        response.feed
    }

    fn parse_urls_from_posts() -> Vec<String> {
        let posts: Vec<Value> = request_bulk_posts_from_api();
        let mut feed: Vec<String> = Vec::with_capacity(config::POSTS_PER_REQUEST);
        for post in posts {
            let at_uri: &str = post["post"]["uri"].as_str().unwrap();
            let http_url: String = convert_at_uri_to_url(at_uri);
            println!("this is a post url {:#?}", http_url);
            feed.push(http_url);
        }
        feed
    }
}

fn main() {
    let total_posts: usize = get_posts_number();

    collect_api_responses(total_posts);
}
