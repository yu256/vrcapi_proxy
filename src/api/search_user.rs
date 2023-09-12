use super::utils::{find_matched_data, request};
use crate::split_colon;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct User {
    #[serde(default)]
    currentAvatarThumbnailImageUrl: String,
    displayName: String,
    id: String,
    isFriend: bool,
    #[serde(default)]
    statusDescription: String,
    #[serde(default)]
    userIcon: String,
    #[serde(default)]
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
        let img = if !user.userIcon.is_empty() {
            user.userIcon
        } else if !user.profilePicOverride.is_empty() {
            user.profilePicOverride
        } else {
            user.currentAvatarThumbnailImageUrl
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
pub(crate) fn api_search_user(req: &str) -> anyhow::Result<Vec<ResUser>> {
    split_colon!(req, [auth, user]);

    let token = find_matched_data(auth)?.1;

    match request(
        "GET",
        &format!("https://api.vrchat.cloud/api/1/users?search={}&n=100", user),
        &token,
    )?
    .into_json::<Vec<User>>()
    {
        Ok(user) => Ok(user.into_iter().map(ResUser::from).collect()),
        Err(err) => Err(err.into()),
    }
}
