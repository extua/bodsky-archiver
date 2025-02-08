#![allow(unused_imports)]
#![allow(dead_code)]
mod bluesky;
use bluesky::get_bluesky_posts;
mod twitter;
use twitter::get_twitter_posts;

fn main() {
    get_bluesky_posts();
    // get_twitter_posts();
}
