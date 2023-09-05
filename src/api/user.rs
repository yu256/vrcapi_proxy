use super::{
    utils::{find_matched_data, request},
    FRIENDS,
};
use crate::{
    api::response::ApiResponse,
    consts::{INVALID_AUTH, VRC_P},
    split_colon,
};
use anyhow::Context as _;
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.vrchat.cloud/api/1/users/";

#[allow(non_snake_case)]
#[derive(Deserialize, Clone)]
pub(crate) struct User {
    pub(crate) bio: Option<String>,
    pub(crate) bioLinks: Vec<String>,
    pub(crate) currentAvatarThumbnailImageUrl: String,
    pub(crate) displayName: String,
    pub(crate) id: String,
    pub(crate) isFriend: bool,
    pub(crate) location: String,
    pub(crate) status: String,
    pub(crate) statusDescription: String,
    pub(crate) tags: Vec<String>,
    pub(crate) userIcon: String,
    pub(crate) profilePicOverride: String,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(crate) struct ResUser {
    bio: Option<String>,
    bioLinks: Vec<String>,
    currentAvatarThumbnailImageUrl: String,
    displayName: String,
    isFriend: bool,
    location: String,
    status: String,
    statusDescription: String,
    rank: String,
}

#[post("/user", data = "<req>")]
pub(crate) async fn api_user(req: &str) -> ApiResponse<ResUser> {
    (|| async {
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
        request("GET", &format!("{}{}", URL, user), &token)
            .map(|res| Ok(res.into_json::<User>()?.into()))?
    })()
    .await
    .into()
}

impl From<User> for ResUser {
    fn from(user: User) -> Self {
        let mut rank = {
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

            rank.unwrap_or("Visitor").to_owned()
        };

        if user.tags.iter().any(|tag| tag == VRC_P) {
            rank += " VRC+"
        }

        ResUser {
            currentAvatarThumbnailImageUrl: user.get_img(),
            bio: user.bio,
            bioLinks: user.bioLinks,
            displayName: user.displayName,
            isFriend: user.isFriend,
            location: user.location,
            status: user.status,
            statusDescription: user.statusDescription,
            rank,
        }
    }
}
