use crate::fetcher::{request, ResponseExt as _};
use crate::global::USERS;
use crate::user::{Status, User};
use crate::validate::validate;
use anyhow::{bail, Result};
use axum::Json;
use hyper::Method;
use serde::Serialize;

const URL: &str = "https://api.vrchat.cloud/api/1/users/";

#[derive(serde::Deserialize)]
pub(crate) struct Query {
    auth: String,
    user_id: Option<String>, // 与えられなかった場合は自身のプロフィールを参照する
    #[serde(default)]
    force: bool,
}

pub(crate) async fn api_user(
    Json(Query {
        auth,
        user_id,
        force,
    }): Json<Query>,
) -> Result<ResUser> {
    let token = validate(auth)?.await;
    match (user_id, force) {
        (Some(user_id), true) => {
            request(Method::GET, &format!("{URL}{user_id}"), &token).await?.json::<User>().await.map(|mut json| {
                json.unsanitize();
                json.into()
            })
        }
        (Some(user_id), false) => {
            let friends = &USERS
            .read()
            .await;
            match friends.online.iter().chain(&friends.web).chain(&friends.offline).find(|u| u.id == user_id).cloned() {
                Some(user) => Ok(user.into()),
                None => {
                    request(Method::GET, &format!("{URL}{user_id}"), &token).await?.json::<User>().await.map(|mut json| {
                        json.unsanitize();
                        json.into()
                    })
                },
            }
        }
        _ => {
            match USERS.read().await.myself.clone() {
                Some(mut user) => Ok({
                    user.unsanitize();
                    user.into()
                }),
                None => bail!("プロフィールの取得に失敗しました。トークンが無効か、ユーザー情報の取得が完了していません。後者の場合は、オンラインになると取得されます。"),
            }
        }
    }
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(crate) struct ResUser {
    id: String,
    bio: String,
    bioLinks: Vec<String>,
    currentAvatarThumbnailImageUrl: String,
    displayName: String,
    isFriend: bool,
    location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    travelingToLocation: Option<String>,
    status: Status,
    statusDescription: String,
    rank: String,
    #[serde(skip_serializing_if = "str::is_empty")]
    userIcon: String,
    #[serde(skip_serializing_if = "str::is_empty")]
    profilePicOverride: String,
}

impl From<User> for ResUser {
    fn from(user: User) -> Self {
        let mut rank = user
            .tags
            .iter()
            .rev()
            .find_map(|tag| match tag.as_str() {
                "system_trust_veteran" => Some("Trusted"),
                "system_trust_trusted" => Some("Known"),
                "system_trust_known" => Some("User"),
                "system_trust_basic" => Some("New User"),
                "system_troll" => Some("Troll"),
                _ => None,
            })
            .unwrap_or("Visitor")
            .to_owned();

        if user.tags.iter().any(|tag| tag == "system_supporter") {
            rank += " VRC+"
        }

        ResUser {
            id: user.id,
            currentAvatarThumbnailImageUrl: user.currentAvatarThumbnailImageUrl,
            bio: user.bio,
            bioLinks: user.bioLinks,
            displayName: user.displayName,
            isFriend: user.isFriend,
            location: user.location,
            travelingToLocation: user.travelingToLocation,
            status: user.status,
            statusDescription: user.statusDescription,
            rank,
            userIcon: user.userIcon,
            profilePicOverride: user.profilePicOverride,
        }
    }
}
