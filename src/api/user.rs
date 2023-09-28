use super::utils::{find_matched_data, request};
use crate::global::FRIENDS;
use crate::websocket::User;
use crate::{get_img, global::INVALID_AUTH, split_colon};
use anyhow::{Context as _, Result};
use serde::Serialize;
use trie_match::trie_match;

const URL: &str = "https://api.vrchat.cloud/api/1/users/";

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(crate) struct ResUser {
    bio: String,
    bioLinks: Vec<String>,
    currentAvatarThumbnailImageUrl: String,
    displayName: String,
    isFriend: bool,
    location: String,
    travelingToLocation: Option<String>,
    status: String,
    statusDescription: String,
    rank: String,
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
    split_colon!(req, [auth, user]);

    if let Some(user) = FRIENDS
        .read()
        .await
        .get(auth)
        .context(INVALID_AUTH)?
        .iter()
        .find(|u| u.id == user)
    {
        return Ok(user.clone().into());
    }

    let token = unsafe { find_matched_data(auth).unwrap_unchecked().1 };
    match request("GET", &format!("{}{}", URL, user), &token)?.into_json::<User>() {
        Ok(json) => Ok(json.into()),
        Err(err) => Err(err.into()),
    }
}
