pub fn convert_at_uri_to_url(at_uri: &str) -> String {
    let did: &str = &at_uri[5..37];
    let rkey: &str = &at_uri[57..];
    let http_url: String = format!("https://bsky.app/profile/{did}/post/{rkey}");
    http_url
}

pub fn posts_per_api_calls_needed(total_posts: usize, posts_per_request: usize) -> Vec<usize> {
    let api_calls_necessary: usize = total_posts.div_ceil(posts_per_request);
    let mut api_call_vec: Vec<usize> = Vec::with_capacity(api_calls_necessary);
    let last_request_remaining_posts: usize = total_posts.rem_euclid(posts_per_request);
    // if total_posts is less than default_posts_per_request
    // return total_posts
    // if total_posts divides cleanly into default_posts_per_request
    // return a vec of default_posts_per_request
    // if total posts does not divide cleanly into default_posts_per_request
    // add default posts per request as above, then add the remainder
    // at the end
    if posts_per_request >= total_posts {
        api_call_vec.push(total_posts);
    } else if last_request_remaining_posts == 0 {
        for _ in 0..api_calls_necessary {
                    api_call_vec.push(posts_per_request)
        }
    } else {
        for _ in 0..(api_calls_necessary -1) {
            api_call_vec.push(posts_per_request)
        }
        api_call_vec.push(last_request_remaining_posts)
    }
    api_call_vec
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_at_uri_to_url() {
        let at_uri: &str = "at://did:plc:blxilps4iwbxicionf2rztej/app.bsky.feed.post/3ld4qc7ixms23";
        let http_url: &str =
            "https://bsky.app/profile/did:plc:blxilps4iwbxicionf2rztej/post/3ld4qc7ixms23";
        let at_uri_converted: String = convert_at_uri_to_url(at_uri);
        assert_eq!(at_uri_converted, http_url);
    }
}