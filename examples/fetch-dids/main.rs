use core::panic;
mod profile;

const ACCOUNT_DID: &str = "did:plc:blxilps4iwbxicionf2rztej";

fn get_posts_number() -> u64 {
    #[tokio::main]
    async fn request_posts_number() -> Result<profile::Root, reqwest::Error> {
        let raw_response: Result<profile::Root, reqwest::Error> = reqwest::Client::new()
            .get("https://public.api.bsky.app/xrpc/app.bsky.actor.getProfile")
            .query(&[("actor", ACCOUNT_DID)])
            .send()
            .await?
            .json::<profile::Root>()
            .await;
        raw_response
    }
    let response: profile::Root = match request_posts_number() {
        Ok(file) => file,
        Err(error) => panic!("Failed to get or parse API response: {error:?}"),
    };
    response.posts_count
}

fn main() {
    let post_count = get_posts_number();
    println!("{:?}", post_count);
}
