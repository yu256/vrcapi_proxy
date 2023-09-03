use super::{user::User, utils::request};
use crate::{api::response::ApiResponse, consts::VRC_P, into_err};
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

#[post("/friends", data = "<req>")]
pub(crate) async fn api_friends(req: &str) -> (Status, Json<ApiResponse<Vec<ResFriend>>>) {
    match get_friends(req).await {
        Ok(friends) => (Status::Ok, Json(friends.into())),

        Err(e) => (Status::InternalServerError, Json(into_err!(e))),
    }
}

pub(crate) async fn fetch_friends(token: &str) -> Result<Vec<User>> {
    let res = request("GET", URL, token)?;

    if res.status() == 200 {
        Ok(res.into_json()?)
    } else {
        bail!("{}", res.into_string()?)
    }
}

async fn get_friends(req: &str) -> Result<Vec<ResFriend>> {
    let read = FRIENDS.read().await;
    let friends = read.get(req).with_context(|| format!("{req}での認証に失敗しました。サーバー側の初回fetchに失敗しているか、トークンが無効です。"))?;

    let mut friends = friends.iter().map(User::to_friend).collect::<Vec<_>>();

    friends.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(friends)
}
