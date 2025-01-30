#![allow(unused_imports)]
#![allow(dead_code)]
use bluesky::get_bluesky_posts;
use twitter::get_twitter_posts;
mod bluesky;
mod twitter;

fn main() {
    get_bluesky_posts();
    // get_twitter_posts();
}