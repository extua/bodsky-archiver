use chrono::{prelude::*, Months};
use core::panic;
use reqwest::{Client, Response, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::time::Duration;
use tokio::time::sleep;
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
    async fn request_profile_from_api(app_client: Client) -> Profile {
        let mut retries: u8 = 0;
        let mut backoff: Duration = Duration::from_secs(1);

        let response_from_retry: Result<Response, reqwest::Error> = loop {
            match app_client
                .get("https://public.api.bsky.app/xrpc/app.bsky.actor.getProfile")
                .query(&[("actor", ACCOUNT_DID)])
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
        // parse the response into profile struct
        let parsed_response: Profile = match response.json::<Profile>().await {
            Ok(response) => response,
            Err(parse_error) => panic!("Failed to parse API response: {parse_error:?}"),
        };
        parsed_response
    }

    let app_client: Client = create_bodsky_client();
    let profile = request_profile_from_api(app_client);
    profile.posts_count
}

fn collect_api_responses(crawl_datetime: DateTime<Utc>, total_posts: usize) -> Vec<String> {
    let posts_per_api_calls_needed: Vec<usize> =
        posts_per_api_calls_needed(total_posts, POSTS_PER_REQUEST);
    // This loop tracks the number of posts remaining
    // and the number to make in each api call
    let mut cursor: String = "".to_string();
    let mut feed: Vec<String> = Vec::with_capacity(total_posts);

    'outer: for posts_to_request in posts_per_api_calls_needed {
        println!("requesting {posts_to_request} posts");
        let bulk_posts: AuthorFeed = request_bulk_posts_from_api(posts_to_request, &cursor);

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

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AuthorFeed {
        cursor: String,
        feed: Vec<Value>,
    }

    #[tokio::main]
    async fn request_bulk_posts_from_api(posts_to_request: usize, cursor: &str) -> AuthorFeed {
        let posts_per_request_str: String = posts_to_request.to_string();

        let app_client = create_bodsky_client();

        let raw_response: Result<AuthorFeed, reqwest::Error> = app_client
            .get("https://public.api.bsky.app/xrpc/app.bsky.feed.getAuthorFeed")
            .query(&[
                ("actor", ACCOUNT_DID),
                ("limit", &posts_per_request_str),
                ("cursor", cursor),
            ])
            .send()
            .await
            .expect("Network issue when making HTTP request")
            .json::<AuthorFeed>()
            .await;
        let response: AuthorFeed = match raw_response {
            Ok(response) => response,
            Err(error) => panic!("Failed to parse API response: {error:?}"),
        };
        response
    }

    feed
}

// Everything orchestrated in this function
// gets exported to main.rs
pub fn get_bluesky_posts() {
    let months_to_go_back: Months = Months::new(5);
    let crawl_datetime: DateTime<Utc> = Utc::now().checked_sub_months(months_to_go_back).unwrap();

    let total_posts: usize = get_posts_number();
    println!("there are {} posts to request", total_posts);
    let feed_urls: Vec<String> = collect_api_responses(crawl_datetime, total_posts);
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
