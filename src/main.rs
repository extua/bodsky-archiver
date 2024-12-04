#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let echo_json: serde_json::Value = reqwest::Client::new()
        .get("https://public.api.bsky.app/xrpc/app.bsky.feed.searchPosts")
        .query(&[("q", "Oxford"), ("limit", "4")])
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}",echo_json);
    Ok(())
}
