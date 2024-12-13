use core::panic;
use serde::{Deserialize, Serialize};
use serde_json::Value;
mod config;

fn get_posts_number() -> u64 {
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Profile {
        posts_count: u64,
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

fn collect_api_responses(api_loops_needed: u64) {
    #[tokio::main]
    async fn request_bulk_posts_from_api() -> Vec<Value> {
        #[derive(Serialize, Deserialize, Debug)]
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
        let feed: Vec<Value> = response.feed;
        feed
    }

    let mut posts = request_bulk_posts_from_api();

    for post in posts.iter_mut() {
        println!("this is a post {:#?}", post);
    }
}

fn main() {
    let api_loops_needed: u64 =
        get_posts_number().div_euclid(config::POSTS_PER_REQUEST.try_into().unwrap()) + 1;
    println!("{}", api_loops_needed);

    collect_api_responses(api_loops_needed);
}
