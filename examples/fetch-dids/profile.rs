use serde::{Serialize,Deserialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub did: String,
    pub handle: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub avatar: Option<String>,
    pub banner: Option<String>,
    pub followers_count: Option<u64>,
    pub follows_count: Option<u64>,
    pub posts_count: u64,
    pub associated: Option<Associated>,
    pub joined_via_starter_pack: Option<JoinedViaStarterPack>,
    pub indexed_at: Option<String>,
    pub created_at: Option<String>,
    pub viewer: Option<Viewer4>,
    pub labels: Option<Vec<Label8>>,
    pub pinned_post: Option<PinnedPost>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Associated {
    pub lists: i64,
    pub feedgens: i64,
    pub starter_packs: i64,
    pub labeler: bool,
    pub chat: Option<Chat>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    pub allow_incoming: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinedViaStarterPack {
    pub uri: String,
    pub cid: String,
    pub record: Record,
    pub creator: Creator,
    pub list_item_count: i64,
    pub joined_week_count: i64,
    pub joined_all_time_count: i64,
    pub labels: Vec<Label4>,
    pub indexed_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Creator {
    pub did: String,
    pub handle: String,
    pub display_name: String,
    pub avatar: String,
    pub associated: Associated2,
    pub viewer: Viewer,
    pub labels: Vec<Label3>,
    pub created_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Associated2 {
    pub lists: i64,
    pub feedgens: i64,
    pub starter_packs: i64,
    pub labeler: bool,
    pub chat: Chat2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chat2 {
    pub allow_incoming: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Viewer {
    pub muted: bool,
    pub muted_by_list: MutedByList,
    pub blocked_by: bool,
    pub blocking: String,
    pub blocking_by_list: BlockingByList,
    pub following: String,
    pub followed_by: String,
    pub known_followers: KnownFollowers,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutedByList {
    pub uri: String,
    pub cid: String,
    pub name: String,
    pub purpose: String,
    pub avatar: String,
    pub list_item_count: i64,
    pub labels: Vec<Label>,
    pub viewer: Viewer2,
    pub indexed_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub ver: i64,
    pub src: String,
    pub uri: String,
    pub cid: String,
    pub val: String,
    pub neg: bool,
    pub cts: String,
    pub exp: String,
    pub sig: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Viewer2 {
    pub muted: bool,
    pub blocked: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockingByList {
    pub uri: String,
    pub cid: String,
    pub name: String,
    pub purpose: String,
    pub avatar: String,
    pub list_item_count: i64,
    pub labels: Vec<Label2>,
    pub viewer: Viewer3,
    pub indexed_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label2 {
    pub ver: i64,
    pub src: String,
    pub uri: String,
    pub cid: String,
    pub val: String,
    pub neg: bool,
    pub cts: String,
    pub exp: String,
    pub sig: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Viewer3 {
    pub muted: bool,
    pub blocked: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KnownFollowers {
    pub count: i64,
    pub followers: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label3 {
    pub ver: i64,
    pub src: String,
    pub uri: String,
    pub cid: String,
    pub val: String,
    pub neg: bool,
    pub cts: String,
    pub exp: String,
    pub sig: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label4 {
    pub ver: i64,
    pub src: String,
    pub uri: String,
    pub cid: String,
    pub val: String,
    pub neg: bool,
    pub cts: String,
    pub exp: String,
    pub sig: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Viewer4 {
    pub muted: bool,
    pub muted_by_list: MutedByList2,
    pub blocked_by: bool,
    pub blocking: String,
    pub blocking_by_list: BlockingByList2,
    pub following: String,
    pub followed_by: String,
    pub known_followers: KnownFollowers2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MutedByList2 {
    pub uri: String,
    pub cid: String,
    pub name: String,
    pub purpose: String,
    pub avatar: String,
    pub list_item_count: i64,
    pub labels: Vec<Label5>,
    pub viewer: Viewer5,
    pub indexed_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label5 {
    pub ver: i64,
    pub src: String,
    pub uri: String,
    pub cid: String,
    pub val: String,
    pub neg: bool,
    pub cts: String,
    pub exp: String,
    pub sig: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Viewer5 {
    pub muted: bool,
    pub blocked: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockingByList2 {
    pub uri: String,
    pub cid: String,
    pub name: String,
    pub purpose: String,
    pub avatar: String,
    pub list_item_count: i64,
    pub labels: Vec<Label6>,
    pub viewer: Viewer6,
    pub indexed_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label6 {
    pub ver: i64,
    pub src: String,
    pub uri: String,
    pub cid: String,
    pub val: String,
    pub neg: bool,
    pub cts: String,
    pub exp: String,
    pub sig: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Viewer6 {
    pub muted: bool,
    pub blocked: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KnownFollowers2 {
    pub count: i64,
    pub followers: Vec<Follower>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Follower {
    pub did: String,
    pub handle: String,
    pub display_name: String,
    pub avatar: String,
    pub associated: Associated3,
    pub labels: Vec<Label7>,
    pub created_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Associated3 {
    pub lists: i64,
    pub feedgens: i64,
    pub starter_packs: i64,
    pub labeler: bool,
    pub chat: Chat3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chat3 {
    pub allow_incoming: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label7 {
    pub ver: i64,
    pub src: String,
    pub uri: String,
    pub cid: String,
    pub val: String,
    pub neg: bool,
    pub cts: String,
    pub exp: String,
    pub sig: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label8 {
    pub ver: i64,
    pub src: String,
    pub uri: String,
    pub cid: String,
    pub val: String,
    pub neg: bool,
    pub cts: String,
    pub exp: String,
    pub sig: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PinnedPost {
    pub uri: String,
    pub cid: String,
}
