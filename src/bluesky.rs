use anyhow::{bail, Result};
use chrono::{prelude::*, Months};
use core::panic;
use reqwest::header::{HeaderMap, HeaderValue, RETRY_AFTER};
use reqwest::{Client, Response, StatusCode, Url};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;
use std::{fs, process};
use tokio::time::sleep;

// local libraries
use bodsky_archiver::call_api;

const ACCOUNT_DID: &str = "bodleianlibraries.bsky.social";
const POSTS_PER_REQUEST: usize = 85;

fn convert_at_uri_to_url(at_uri: &str) -> String {
    let did: &str = &at_uri[5..37];
    let rkey: &str = &at_uri[57..];
    let http_url: String = format!("https://bsky.app/profile/{did}/post/{rkey}");
    http_url
}

fn posts_per_api_calls_needed(total_posts: usize, posts_per_request: usize) -> Vec<usize> {
    let api_calls_necessary: usize = total_posts.div_ceil(posts_per_request);
    let mut api_call_vec: Vec<usize> = Vec::with_capacity(api_calls_necessary);
    let last_request_remaining_posts: usize = total_posts.rem_euclid(posts_per_request);
    // if total_posts is less than default_posts_per_request
    // return total_posts
    // if total_posts divides cleanly into default_posts_per_request
    // return a vec of default_posts_per_request
    // if total posts does not divide cleanly into default_posts_per_request
    // add default posts per request as above, then add the remainder
    // at the end
    if posts_per_request >= total_posts {
        api_call_vec.push(total_posts);
    } else if last_request_remaining_posts == 0 {
        for _ in 0..api_calls_necessary {
            api_call_vec.push(posts_per_request)
        }
    } else {
        for _ in 0..(api_calls_necessary - 1) {
            api_call_vec.push(posts_per_request)
        }
        api_call_vec.push(last_request_remaining_posts)
    }
    api_call_vec
}

fn create_bodsky_client() -> Client {
    const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
    reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .gzip(true)
        .build()
        .expect("unable to create client")
}

#[tokio::main]
async fn get_posts_number(app_client: &Client) -> Result<usize> {
    // This function gets the number of posts
    // posted by a given account 'did', from
    // an actor.getProfile api call

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Profile {
        posts_count: usize,
    }

    let endpoint = Url::parse_with_params(
        "https://public.api.bsky.app/xrpc/app.bsky.actor.getProfile",
        &[("actor", ACCOUNT_DID)],
    )
    .unwrap();

    let response: String = call_api(app_client, endpoint)?;

    // parse the response into tweet struct
    let profile: Profile = serde_json::from_str(&response)?;

    Ok(profile.posts_count)
}

fn collect_api_responses(
    crawl_datetime: DateTime<Utc>,
    total_posts: usize,
    app_client: &Client,
) -> Result<Vec<String>> {
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AuthorFeed {
        cursor: String,
        feed: Vec<Value>,
    }

    let posts_per_api_calls_needed: Vec<usize> =
        posts_per_api_calls_needed(total_posts, POSTS_PER_REQUEST);
    // This loop tracks the number of posts remaining
    // and the number to make in each api call
    let mut cursor: String = "".to_string();
    let mut feed: Vec<String> = Vec::with_capacity(total_posts);

    'outer: for posts_to_request in posts_per_api_calls_needed {

        println!("requesting {posts_to_request} posts");

        
        let endpoint = Url::parse_with_params(
            "https://public.api.bsky.app/xrpc/app.bsky.actor.getProfile",
            &[
                ("actor", ACCOUNT_DID),
                ("limit", &posts_to_request.to_string()),
                ("cursor", &cursor),
            ],
        )
        .unwrap();

        let response: String = call_api(app_client, endpoint)?;

        // parse the response into tweet struct
        let bulk_posts: AuthorFeed = serde_json::from_str(&response)?;

        // update the cursor value
        cursor = bulk_posts.cursor;

        let cursor_rfc3399: DateTime<Utc> = DateTime::parse_from_rfc3339(&cursor).unwrap().to_utc();
        println!("crawl datetime is {}", crawl_datetime);
        println!("cursor is         {}", cursor_rfc3399);

        for post in bulk_posts.feed {
            let at_uri: &str = post["post"]["uri"].as_str().unwrap();
            let http_url: String = convert_at_uri_to_url(at_uri);
            let index_timestamp: &str = post["post"]["indexedAt"].as_str().unwrap();
            let index_timestamp_parsed: DateTime<Utc> =
                DateTime::parse_from_rfc3339(index_timestamp)
                    .unwrap()
                    .to_utc();
            // At this point, check whether the indexed timestamp
            // is greater than last crawl timestamp, and if so,
            // break all the way out of the outer loop
            if index_timestamp_parsed <= crawl_datetime {
                break 'outer;
            }
            feed.push(http_url);
        }
    }
    Ok(feed)
}

// Everything orchestrated in this function
// gets exported to main.rs
pub fn get_bluesky_posts() {
    let months_to_go_back: Months = Months::new(5);
    let crawl_datetime: DateTime<Utc> = Utc::now().checked_sub_months(months_to_go_back).unwrap();

    let app_client: Client = create_bodsky_client();

    let total_posts = get_posts_number(&app_client).unwrap_or_else(|error| {
        // exit to stderr
        eprintln!("Failed to collect number of bluesky posts in account: {error}");
        process::exit(1)
    });

    println!("there are {} posts to request", total_posts);

    let feed_urls =
        collect_api_responses(crawl_datetime, total_posts, &app_client).unwrap_or_else(|error| {
            // exit to stderr
            eprintln!("Failed to collect bluesky posts: {error}");
            process::exit(1)
        });

    println!("collected {} posts", feed_urls.len());

    // Now write everything out to a file
    let account_did: &str = ACCOUNT_DID;
    let timestamp: String = Utc::now().timestamp().to_string();
    let file_path: String = format!("{account_did}-{timestamp}.txt");
    fs::write(file_path, feed_urls.join("\n")).expect("unable to write to file");
}

// Tests here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_at_uri_to_url() {
        let at_uri: &str = "at://did:plc:blxilps4iwbxicionf2rztej/app.bsky.feed.post/3ld4qc7ixms23";
        let http_url: &str =
            "https://bsky.app/profile/did:plc:blxilps4iwbxicionf2rztej/post/3ld4qc7ixms23";
        let at_uri_converted: String = convert_at_uri_to_url(at_uri);
        assert_eq!(at_uri_converted, http_url);
    }

    #[test]
    fn test_posts_per_api_calls_needed() {
        for total_posts in 0..400 {
            for posts_per_request in 1..100 {
                let api_call_vec: Vec<usize> =
                    posts_per_api_calls_needed(total_posts, posts_per_request);
                let total_posts_from_function: usize = api_call_vec.iter().sum();
                assert_eq!(total_posts, total_posts_from_function);
            }
        }
    }
}
