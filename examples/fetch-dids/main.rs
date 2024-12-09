use core::panic;
use std::error::Error;
use serde::Deserialize;

use serde_json::{Number, Value};
const ACCOUNT_DID: &str = "did:plc:blxilps4iwbxicionf2rztej";

#[tokio::main]
async fn get_posts_number() -> Result<Value, reqwest::Error> {
    // #[derive(Deserialize)]
    // struct postsCount {
    //     postsCount: Number,
    // }

    let echo_json: serde_json::Value = reqwest::Client::new()
        .get("https://public.api.bsky.app/xrpc/app.bsky.actor.getProfile")
        .query(&[("actor", ACCOUNT_DID)])
        .send()
        .await?
        .json()
        .await?;

    Ok(echo_json)
}

// let post_count: Option<u64> = get_posts_number()["postsCount"].as_u64();

fn main() {
    let post_count = get_posts_number().expect("fasfsafa");

    // let post_count: u64 = match get_posts_number() {
    //     Some (post_count) => post_count,
    //     None => 0u64,
    //     };
    println!("{}", post_count);
}

// let echo_json: serde_json::Value = reqwest::Client::new()
//     .get("https://public.api.bsky.app/xrpc/app.bsky.feed.getAuthorFeed")
//     .query(&[("actor", "did:plc:blxilps4iwbxicionf2rztej"), ("limit", "100"), ("cursor", "2023-11-28T11:40:31.232Z")])
//     .send()
//     .await?
//     .json()
//     .await?;

// println!("{echo_json:#?}");
// Ok(())
// }

// function returns object, cursor
