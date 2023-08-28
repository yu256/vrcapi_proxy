use super::{user::User, utils::request};
use crate::{consts::VRC_P, split_colon};
use anyhow::{bail, Context as _, Result};
use rocket::{http::Status, serde::json::Json, tokio::sync::RwLock};
use serde::Serialize;
use std::{collections::HashMap, sync::LazyLock};

pub(crate) static FRIENDS: LazyLock<RwLock<HashMap<String, Vec<User>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

const URL: &str = "https://api.vrchat.cloud/api/1/auth/user/friends?offline=false";

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(crate) struct ResFriend {
    currentAvatarThumbnailImageUrl: String,
    id: String,
    status: String,
    location: String,
}

impl User {
    pub(crate) fn to_friend(&self) -> ResFriend {
        ResFriend {
            currentAvatarThumbnailImageUrl: self.get_img(),
            id: self.id.to_owned(),
            status: self.status.to_owned(),
            location: self.location.to_owned(),
        }
    }

    pub(crate) fn get_img(&self) -> String {
        let img = match self.tags.iter().any(|tag| tag == VRC_P) {
            true if !self.userIcon.is_empty() => &self.userIcon,
            true if !self.profilePicOverride.is_empty() => &self.profilePicOverride,
            _ => &self.currentAvatarThumbnailImageUrl,
        };
        img.to_owned()
    }
}

#[derive(Serialize)]
pub(crate) enum Response {
    Success(Vec<ResFriend>),
    Error(String),
}

#[post("/friends", data = "<req>")]
pub(crate) async fn api_friends(req: &str) -> (Status, Json<Response>) {
    match get_friends(req).await {
        Ok(friends) => (Status::Ok, Json(Response::Success(friends))),

        Err(e) => (
            Status::InternalServerError,
            Json(Response::Error(e.to_string())),
        ),
    }
}

pub(crate) async fn fetch_friends(token: &str) -> Result<Vec<User>> {
    let res = request(reqwest::Method::GET, URL, token).await?;

    if res.status().is_success() {
        Ok(res.json().await?)
    } else {
        bail!("{}", res.text().await?)
    }
}

async fn get_friends(req: &str) -> Result<Vec<ResFriend>> {
    let mut friends = {
        split_colon!(req, [auth, askme]);
        let read = FRIENDS.read().await;
        let friends = read.get(auth).context("failed to auth.")?;
        friends
            .iter()
            .filter(|friend| {
                friend.location != "offline" && (askme == "true" || friend.status != "ask me")
            })
            .map(User::to_friend)
            .collect::<Vec<_>>()
    };

    friends.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(friends)
}
