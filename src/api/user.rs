use super::{
    utils::{find_matched_data, request},
    FRIENDS,
};
use crate::{
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
    pub(crate) bioLinks: Option<Vec<String>>,
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
    #[serde(default)]
    pub(crate) undetermined: bool,
}

impl User {
    pub(crate) fn get_img(&self) -> String {
        let img = match self.tags.iter().any(|tag| tag == VRC_P) {
            true if !self.userIcon.is_empty() => &self.userIcon,
            true if !self.profilePicOverride.is_empty() => &self.profilePicOverride,
            _ => &self.currentAvatarThumbnailImageUrl,
        };
        img.to_owned()
    }
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(crate) struct ResUser {
    bio: Option<String>,
    bioLinks: Option<Vec<String>>,
    currentAvatarThumbnailImageUrl: String,
    displayName: String,
    isFriend: bool,
    location: String,
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

#[post("/user", data = "<req>")]
pub(crate) async fn api_user(req: &str) -> anyhow::Result<ResUser> {
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
