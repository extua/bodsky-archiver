use atrium_api::{
    agent::{store::MemorySessionStore, AtpAgent},
    com::atproto::sync::list_repos::ParametersData,
};
use atrium_xrpc_client::reqwest::ReqwestClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let agent = AtpAgent::new(
        ReqwestClient::new("https://bsky.social"),
        MemorySessionStore::default(),
    );
    dotenvy::dotenv()?;
    let password: String = env::var("BSKY_PASSWORD").expect("Password must be set in .env file.");
    let user_id: String = env::var("BSKY_USER").expect("User id must be set in .env file.");
    agent.login(user_id, password).await?;

    let result = agent
        .api
        .com
        .atproto
        .sync
        .list_repos(
            ParametersData {
                cursor: None,
                limit: None,
            }
            .into(),
        )
        .await?;
    println!("{:?}", result);
    Ok(())
}
