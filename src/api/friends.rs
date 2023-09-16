use super::user::User;
use crate::consts::INVALID_AUTH;
use anyhow::{Context as _, Result};
use rocket::tokio::sync::RwLock;
use serde::Serialize;
use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

pub(crate) static FRIENDS: LazyLock<RwLock<HashMap<String, Vec<User>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));
pub(crate) static FAVORITE_FRIENDS: LazyLock<RwLock<HashMap<String, HashSet<String>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(crate) struct Friend {
    currentAvatarThumbnailImageUrl: String,
    id: String,
    status: String,
    location: String,
    undetermined: bool,
}

#[derive(Serialize)]
pub(crate) struct ResFriend {
    public: Vec<Friend>,
    private: Vec<Friend>,
}

impl From<&User> for Friend {
    fn from(user: &User) -> Self {
        Self {
            currentAvatarThumbnailImageUrl: user.get_img(),
            id: user.id.to_owned(),
            status: user.status.to_owned(),
            location: user.location.to_owned(),
            undetermined: user.undetermined,
        }
    }
}

#[post("/friends", data = "<req>")]
pub(crate) async fn api_friends(req: &str) -> Result<ResFriend> {
    let (public, private) = FRIENDS
        .read()
        .await
        .get(req)
        .context(INVALID_AUTH)?
        .iter()
        .map(Friend::from)
        .partition(|friend| friend.location != "private");

    Ok(ResFriend { public, private })
}

#[post("/favfriends", data = "<req>")]
pub(crate) async fn api_friends_filtered(req: &str) -> Result<ResFriend> {
    let unlocked = FAVORITE_FRIENDS.read().await;
    let favorites = unlocked.get(req).context(INVALID_AUTH)?;
    api_friends(req).await.map(|mut friends| {
        friends
            .private
            .retain(|friend| favorites.contains(&friend.id));
        friends
            .public
            .retain(|friend| favorites.contains(&friend.id));
        friends
    })
}
