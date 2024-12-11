use core::panic;
use serde::{Deserialize, Serialize};

const ACCOUNT_DID: &str = "did:plc:blxilps4iwbxicionf2rztej";

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
            .query(&[("actor", ACCOUNT_DID)])
            .send()
            .await?
            .json::<Profile>()
            .await;
        raw_response
    }
    let response: Profile = match request_profile_from_api() {
        Ok(file) => file,
        Err(error) => panic!("Failed to get or parse API response: {error:?}"),
    };
    response.posts_count
}

fn main() {
    let post_count: u64 = get_posts_number();
    println!("{}", post_count);
}
