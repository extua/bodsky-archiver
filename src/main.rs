#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let echo_json: serde_json::Value = reqwest::Client::new()
        .get("https://public.api.bsky.app/xrpc/app.bsky.feed.getAuthorFeed")
        .query(&[("actor", "did:plc:blxilps4iwbxicionf2rztej")])
        .send()
        .await?
        .json()
        .await?;

    println!("{}",echo_json["feed"][0]["post"]["record"]["text"]);
    Ok(())
}
