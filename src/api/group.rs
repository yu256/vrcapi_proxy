use super::utils::{find_matched_data, request};
use crate::{api::response::ApiResponse, split_colon};
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
    iconId: String,
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
    myMember: Option<Member>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub(crate) struct Gallery {
    id: String,
    name: String,
    description: String,
    membersOnly: bool,
    roleIdsToView: Option<Vec<String>>,
    roleIdsToSubmit: Vec<String>,
    roleIdsToAutoApprove: Vec<String>,
    roleIdsToManage: Vec<String>,
    createdAt: String,
    updatedAt: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub(crate) struct Member {
    id: String,
    groupId: String,
    userId: String,
    roleIds: Vec<String>,
    managerNotes: Option<String>,
    membershipStatus: String,
    isSubscribedToAnnouncements: bool,
    visibility: String,
    isRepresenting: bool,
    joinedAt: String,
    bannedAt: Option<String>,
    has2FA: bool,
    permissions: Vec<String>,
}

#[post("/group", data = "<req>")]
pub(crate) fn api_group(req: &str) -> ApiResponse<Group> {
    (|| {
        split_colon!(req, [auth, id]);

        let token = find_matched_data(auth)?.1;

        request(
            "GET",
            &format!("https://api.vrchat.cloud/api/1/groups/{id}"),
            &token,
        )
        .map(|res| res.into_json::<Group>().map_err(From::from))?
    })()
    .into()
}
