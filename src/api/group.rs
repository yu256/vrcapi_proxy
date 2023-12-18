use super::utils::request;
use crate::{split_colon, validate};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub(crate) struct Group {
    id: String,
    name: String,
    shortCode: String,
    discriminator: String,
    description: String,
    iconUrl: String,
    bannerUrl: String,
    privacy: String,
    ownerId: String,
    rules: String,
    links: Vec<String>,
    languages: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iconId: Option<String>,
    bannerId: String,
    memberCount: i32,
    memberCountSyncedAt: String,
    isVerified: bool,
    joinState: String,
    tags: Vec<String>,
    galleries: Vec<Gallery>,
    createdAt: String,
    onlineMemberCount: i32,
    membershipStatus: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    myMember: Option<Member>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct Gallery {
    id: String,
    name: String,
    description: String,
    membersOnly: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    roleIdsToView: Option<Vec<String>>,
    roleIdsToSubmit: Vec<String>,
    roleIdsToAutoApprove: Vec<String>,
    roleIdsToManage: Vec<String>,
    createdAt: String,
    updatedAt: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct Member {
    id: String,
    groupId: String,
    userId: String,
    roleIds: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managerNotes: Option<String>,
    membershipStatus: String,
    isSubscribedToAnnouncements: bool,
    visibility: String,
    isRepresenting: bool,
    joinedAt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    bannedAt: Option<String>,
    has2FA: bool,
    permissions: Vec<String>,
}

pub(crate) async fn api_group(req: String) -> Result<Group> {
    split_colon!(req, [auth, id]);
    validate!(auth, token);

    request(
        "GET",
        &format!("https://api.vrchat.cloud/api/1/groups/{id}"),
        token,
    )?
    .into_json()
    .map_err(From::from)
}
