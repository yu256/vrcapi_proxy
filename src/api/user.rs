use super::utils::{find_matched_data, request};
use crate::api::utils::request_json;
use crate::global::{FRIENDS, USERS};
use crate::websocket::structs::Status;
use crate::websocket::User;
use crate::{get_img, split_colon};
use anyhow::{anyhow, Result};
use axum::Json;
use serde::{Deserialize, Serialize};
use trie_match::trie_match;

const URL: &str = "https://api.vrchat.cloud/api/1/users/";

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(crate) struct ResUser {
    id: String,
    bio: String,
    bioLinks: Vec<String>,
    currentAvatarThumbnailImageUrl: String,
    displayName: String,
    isFriend: bool,
    location: String,
    travelingToLocation: Option<String>,
    status: Status,
    statusDescription: String,
    rank: String,
    hasUserIcon: bool,
}

impl From<User> for ResUser {
    fn from(user: User) -> Self {
        let mut rank = user
            .tags
            .iter()
            .rev()
            .find_map(|tag| {
                trie_match! {
                    match tag.as_str() {
                        "system_trust_veteran" => Some("Trusted"),
                        "system_trust_trusted" => Some("Known"),
                        "system_trust_known" => Some("User"),
                        "system_trust_basic" => Some("New User"),
                        "system_troll" => Some("Troll"),
                        _ => None,
                    }
                }
            })
            .unwrap_or("Visitor")
            .to_owned();

        if user.tags.iter().any(|tag| tag == "system_supporter") {
            rank += " VRC+"
        }

        ResUser {
            id: user.id,
            hasUserIcon: !user.userIcon.is_empty(),
            currentAvatarThumbnailImageUrl: get_img!(user),
            bio: user.bio,
            bioLinks: user.bioLinks,
            displayName: user.displayName,
            isFriend: user.isFriend,
            location: user.location,
            travelingToLocation: user.travelingToLocation,
            status: user.status,
            statusDescription: user.statusDescription,
            rank,
        }
    }
}

pub(crate) async fn api_user(req: String) -> Result<ResUser> {
    if !req.contains(':') {
        return match USERS.read(&req).await {
            Some(mut user) => Ok({
                user.unsanitize();
                user.into()
            }),
            None => Err(anyhow!("プロフィールの取得に失敗しました。トークンが無効か、ユーザー情報の取得が完了していません。後者の場合は、オンラインになると取得されます。")),
        };
    }

    split_colon!(req, [auth, user]);

    if let Some(user) = FRIENDS
        .read(auth, |friends| {
            friends.iter().find(|u| u.id == user).cloned()
        })
        .await?
    {
        return Ok(user.into());
    }

    // Safety: 存在しない場合FRIENDS.read()で早期returnされるので、必ずOkである
    let token = unsafe { find_matched_data(auth).unwrap_unchecked().1 };
    match request("GET", &format!("{}{}", URL, user), &token)?.into_json::<User>() {
        Ok(mut json) => Ok({
            json.unsanitize();
            json.into()
        }),
        Err(err) => Err(err.into()),
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct ProfileUpdateQuery {
    auth: String,
    user: String,
    query: Query,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone)]
struct Query {
    status: Status,
    statusDescription: String,
    bio: String,
    bioLinks: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    userIcon: Option<String>,
}

pub(crate) async fn api_update_profile(Json(req): Json<ProfileUpdateQuery>) -> Result<bool> {
    request_json(
        "PUT",
        &format!("{}{}", URL, req.user),
        &find_matched_data(&req.auth)?.1,
        req.query.clone(),
    )?;

    USERS
        .write(&req.auth, |user| {
            user.status = req.query.status;
            user.statusDescription = req.query.statusDescription;
            user.bio = req.query.bio;
            user.bioLinks = req.query.bioLinks;
            if let Some(user_icon) = req.query.userIcon {
                user.userIcon = user_icon;
            }
        })
        .await;

    Ok(true)
}
