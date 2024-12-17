

pub fn convert_at_uri_to_url(at_uri: &str) -> String {
    let did: &str = &at_uri[5..37];
    let rkey: &str = &at_uri[57..];
    let http_url: String = format!("https://bsky.app/profile/{did}/post/{rkey}");
    http_url
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