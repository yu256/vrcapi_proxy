use super::utils::{request, StrExt as _};
use crate::general::find_matched_data;
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
    myMember: Member,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub(crate) struct Gallery {
    id: String,
    name: String,
    description: String,
    membersOnly: bool,
    roleIdsToView: Vec<String>,
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

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub(crate) struct Role {
    id: String,
    groupId: String,
    name: String,
    description: String,
    isSelfAssignable: bool,
    permissions: Vec<String>,
    isManagementRole: bool,
    requiresTwoFactor: bool,
    requiresPurchase: bool,
    order: u32,
    createdAt: String,
    updatedAt: String,
}

#[derive(Serialize)]
pub(crate) enum Response {
    Success(Group),
    Error(String),
}

const URL: &str = "https://api.vrchat.cloud/api/1/groups/";

#[post("/group", data = "<req>")]
pub(crate) async fn api_group(req: &str) -> (Status, Json<Response>) {
    match fetch(req).await {
        Ok(grp) => (Status::Ok, Json(Response::Success(grp))),

        Err(error) => (
            Status::InternalServerError,
            Json(Response::Error(error.to_string())),
        ),
    }
}

async fn fetch(req: &str) -> Result<Group> {
    let (auth, id) = req.split_colon()?;

    let matched = find_matched_data(auth)?;

    let res = request(reqwest::Method::GET, &format!("{URL}{id}"), &matched.token).await?;

    if res.status().is_success() {
        let grp: Group = res.json().await?;
        Ok(grp)
    } else {
        bail!("{}", res.status())
    }
}
