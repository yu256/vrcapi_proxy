use super::utils::{find_matched_data, request};
use crate::{api::response::ApiResponse, into_err, split_colon};
use anyhow::{bail, Result};
use rocket::{http::Status, serde::json::Json};
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
pub(crate) async fn api_group(req: &str) -> (Status, Json<ApiResponse<Group>>) {
    match fetch(req).await {
        Ok(grp) => (Status::Ok, Json(grp.into())),

        Err(error) => (Status::InternalServerError, Json(into_err!(error))),
    }
}

async fn fetch(req: &str) -> Result<Group> {
    split_colon!(req, [auth, id]);

    let (_, token) = find_matched_data(auth)?;

    let res = request(
        reqwest::Method::GET,
        &format!("https://api.vrchat.cloud/api/1/groups/{id}"),
        &token,
    )
    .await?;

    if res.status().is_success() {
        Ok(res.json().await?)
    } else {
        bail!("{}", res.text().await?)
    }
}
