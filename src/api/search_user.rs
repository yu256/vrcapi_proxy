use super::utils::{request, StrExt as _};
use crate::{consts::VRC_P, general::find_matched_data};
use anyhow::{bail, Result};
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.vrchat.cloud/api/1/users?search=";

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct User {
    currentAvatarThumbnailImageUrl: String,
    displayName: String,
    id: String,
    isFriend: bool,
    statusDescription: String,
    tags: Vec<String>,
    userIcon: String,
    profilePicOverride: String,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(crate) struct ResUser {
    currentAvatarThumbnailImageUrl: String,
    displayName: String,
    id: String,
    isFriend: bool,
    statusDescription: String,
}

impl From<User> for ResUser {
    fn from(user: User) -> Self {
        ResUser {
            currentAvatarThumbnailImageUrl: if user.tags.iter().all(|tag| tag != VRC_P) {
                user.currentAvatarThumbnailImageUrl
            } else if user.userIcon.is_empty() {
                if user.profilePicOverride.is_empty() {
                    user.currentAvatarThumbnailImageUrl
                } else {
                    user.profilePicOverride
                }
            } else {
                user.userIcon
            },
            displayName: user.displayName,
            id: user.id,
            isFriend: user.isFriend,
            statusDescription: user.statusDescription,
        }
    }
}

#[derive(Serialize)]
pub(crate) enum Response {
    Success(Vec<ResUser>),
    Error(String),
}

#[post("/search_user", data = "<req>")]
pub(crate) async fn api_search_user(req: &str) -> (Status, Json<Response>) {
    match fetch(req).await {
        Ok(users) => (Status::Ok, Json(Response::Success(users))),

        Err(error) => (
            Status::InternalServerError,
            Json(Response::Error(error.to_string())),
        ),
    }
}

async fn fetch(req: &str) -> Result<Vec<ResUser>> {
    let (auth, user) = req.split_colon()?;

    let matched = find_matched_data(auth)?;

    let res = request(
        reqwest::Method::GET,
        &format!("{}{}", URL, user),
        &matched.token,
    )
    .await?;

    if res.status().is_success() {
        let users: Vec<User> = res.json().await?;
        Ok(users.into_iter().map(ResUser::from).collect())
    } else {
        bail!("{}", res.status())
    }
}
