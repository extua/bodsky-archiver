mod bluesky;
use bluesky::get_bluesky_posts;
mod twitter;
#[allow(unused_imports)]
#[allow(dead_code)]
use twitter::get_twitter_posts;

#[tokio::main]
async fn main() {
    get_bluesky_posts().await;
    // get_twitter_posts().await;
}
