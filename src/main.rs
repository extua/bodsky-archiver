mod bluesky;
use bluesky::get_bluesky_posts;
mod twitter;
#[allow(unused_imports)]
#[allow(dead_code)]
use twitter::get_twitter_posts;

#[tokio::main]
async fn main() {
    get_bluesky_posts().await;
    // comment out twitter as actually running this
    // will use precious API invocation calls
    // get_twitter_posts().await;
}
