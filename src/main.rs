use bodsky_archiver::*;
use chrono::prelude::*;
use core::panic;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
const ACCOUNT_DID: &str = "bodleianlibraries.bsky.social";
const POSTS_PER_REQUEST: usize = 85;


fn get_posts_number() -> usize {
    // This function gets the number of posts
    // posted by a given account 'did', from
    // an actor.getProfile api call

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Profile {
        posts_count: usize,
    }

    #[tokio::main]
    async fn request_profile_from_api() -> Result<Profile, reqwest::Error> {
        let raw_response: Result<Profile, reqwest::Error> = reqwest::Client::new()
            .get("https://public.api.bsky.app/xrpc/app.bsky.actor.getProfile")
            .query(&[("actor", ACCOUNT_DID)])
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

fn collect_api_responses(total_posts: usize) -> Vec<String> {
    let posts_per_api_calls_needed: Vec<usize> =
        posts_per_api_calls_needed(total_posts, POSTS_PER_REQUEST);
    // This loop tracks the number of posts remaining and the number
    // to make in each api call
    let mut cursor: String = "".to_string();
    let mut feed: Vec<String> = Vec::with_capacity(total_posts);

    for posts_to_request in posts_per_api_calls_needed {
        println!("requesting {posts_to_request} posts");
        let bulk_posts: AuthorFeed = request_bulk_posts_from_api(posts_to_request, &cursor);
        // update the cursor value
        cursor = bulk_posts.cursor;
        for post in bulk_posts.feed {
            let at_uri: &str = post["post"]["uri"].as_str().unwrap();
            let http_url: String = convert_at_uri_to_url(at_uri);
            feed.push(http_url);
        }
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AuthorFeed {
        cursor: String,
        feed: Vec<Value>,
    }

    #[tokio::main]
    async fn request_bulk_posts_from_api(posts_to_request: usize, cursor: &str) -> AuthorFeed {
        let posts_per_request_str: String = posts_to_request.to_string();

        let raw_response: Result<AuthorFeed, reqwest::Error> = reqwest::Client::new()
            .get("https://public.api.bsky.app/xrpc/app.bsky.feed.getAuthorFeed")
            .query(&[
                ("actor", ACCOUNT_DID),
                ("limit", &posts_per_request_str),
                ("cursor", cursor),
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
        response
    }

    feed
}

fn main() {
    let total_posts: usize = get_posts_number();
    println!("there are {} posts to request", total_posts);
    let feed_urls: Vec<String> = collect_api_responses(total_posts);
    println!("collected {} posts", feed_urls.len());
    // Now write everything out to a file
    let account_did: &str = ACCOUNT_DID;
    let timestamp: String = Utc::now().timestamp().to_string();
    let file_path: String = format!("{account_did}-{timestamp}.txt");
    fs::write(file_path, feed_urls.join("\n")).expect("unable to write to file");
}
