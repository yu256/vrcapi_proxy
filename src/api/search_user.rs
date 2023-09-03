use super::utils::{find_matched_data, request};
use crate::{consts::VRC_P, split_colon, api::response::ApiResponse, into_err};
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
        let img = match user.tags.iter().any(|tag| tag == VRC_P) {
            true if !user.userIcon.is_empty() => user.userIcon,
            true if !user.profilePicOverride.is_empty() => user.profilePicOverride,
            _ => user.currentAvatarThumbnailImageUrl,
        };

        ResUser {
            currentAvatarThumbnailImageUrl: img,
            displayName: user.displayName,
            id: user.id,
            isFriend: user.isFriend,
            statusDescription: user.statusDescription,
        }
    }
}

#[post("/search_user", data = "<req>")]
pub(crate) async fn api_search_user(req: &str) -> (Status, Json<ApiResponse<Vec<ResUser>>>) {
    match fetch(req).await {
        Ok(users) => (Status::Ok, Json(users.into())),

        Err(error) => (
            Status::InternalServerError,
            Json(into_err!(error)),
        ),
    }
}

async fn fetch(req: &str) -> Result<Vec<ResUser>> {
    split_colon!(req, [auth, user]);

    let (_, token) = find_matched_data(auth)?;

    let res = request(reqwest::Method::GET, &format!("{}{}", URL, user), &token).await?;

    if res.status().is_success() {
        Ok(res
            .json::<Vec<User>>()
            .await?
            .into_iter()
            .map(ResUser::from)
            .collect())
    } else {
        bail!("{}", res.text().await?)
    }
}
