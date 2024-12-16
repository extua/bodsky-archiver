Using the [Bodleian Libraries bluesky account](https://bsky.app/profile/bodleianlibraries.bsky.social) as an example


## Resolve handle

```url
https://public.api.bsky.app/xrpc/com.atproto.identity.resolveHandle?handle=bodleianlibraries.bsky.social
```

This endpoint returns the ID `did:plc:blxilps4iwbxicionf2rztej`

## Get posts

Using the ID we found above, we can get the posts associated with the account

```url
https://public.api.bsky.app/xrpc/app.bsky.feed.getAuthorFeed?actor=did:plc:blxilps4iwbxicionf2rztej
```

This endpoint returns a bunch of JSON for the posts.

### Other attempts

Other people have also tried doing this

* [Zhuowei Zhang](https://worthdoingbadly.com/bsky/)
* [Daniel van Strien](https://bsky.app/profile/did:plc:7e5mpxuweopubhexwqg5l3ba/post/3lbu6l4fxdc2e)
* [Filipo Valsorda](https://bsky.app/profile/filippo.abyssdomain.expert/post/3lcfdsv2hec2a)
