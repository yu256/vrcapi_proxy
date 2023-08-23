use super::utils::{request, StrExt as _};
use crate::{consts::VRC_P, general::find_matched_data};
use anyhow::{bail, Result};
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.vrchat.cloud/api/1/users/";

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct User {
    bio: String,
    bioLinks: Vec<String>,
    currentAvatarThumbnailImageUrl: String,
    displayName: String,
    last_activity: Option<String>,
    location: String,
    status: String,
    statusDescription: String,
    tags: Vec<String>,
    userIcon: String,
    profilePicOverride: String,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(crate) struct ResUser {
    bio: String,
    bioLinks: Vec<String>,
    currentAvatarThumbnailImageUrl: String,
    displayName: String,
    last_activity: Option<String>,
    location: String,
    status: String,
    statusDescription: String,
    rank: String,
}

#[derive(Serialize)]
pub(crate) enum Response {
    Success(ResUser),
    Error(String),
}

#[post("/user", data = "<req>")]
pub(crate) async fn api_user(req: &str) -> (Status, Json<Response>) {
    match fetch(req).await {
        Ok(user) => (Status::Ok, Json(Response::Success(user))),

        Err(error) => (
            Status::InternalServerError,
            Json(Response::Error(error.to_string())),
        ),
    }
}

async fn fetch(req: &str) -> Result<ResUser> {
    let (auth, user) = req.split_colon()?;

    let matched = find_matched_data(auth)?;

    let res = request(
        reqwest::Method::GET,
        &format!("{}{}", URL, user),
        &matched.token,
    )
    .await?;

    if res.status().is_success() {
        let user: User = res.json().await?;
        Ok(add_rank(user))
    } else {
        bail!("{}", res.status())
    }
}

fn add_rank(user: User) -> ResUser {
    let mut rank = None;
    for tag in user.tags.iter().rev() {
        match tag.as_str() {
            "system_trust_veteran" => {
                rank = Some("Trusted");
                break;
            }
            "system_trust_trusted" => {
                rank = Some("Known");
                break;
            }
            "system_trust_known" => {
                rank = Some("User");
                break;
            }
            "system_trust_basic" => {
                rank = Some("New User");
                break;
            }
            "system_troll" => {
                rank = Some("Troll");
                break;
            }
            _ => {}
        }
    }

    let is_vrc_p = user.tags.iter().any(|tag| tag == VRC_P);
    let mut rank = rank.unwrap_or_else(|| "Visitor").to_owned();

    if *&is_vrc_p {
        rank += " VRC+"
    }

    let img = match &is_vrc_p {
        true if !user.userIcon.is_empty() => user.userIcon,
        true if !user.profilePicOverride.is_empty() => user.profilePicOverride,
        _ => user.currentAvatarThumbnailImageUrl,
    };

    ResUser {
        bio: user.bio,
        bioLinks: user.bioLinks,
        currentAvatarThumbnailImageUrl: img,
        displayName: user.displayName,
        last_activity: user.last_activity,
        location: user.location,
        status: user.status,
        statusDescription: user.statusDescription,
        rank,
    }
}
