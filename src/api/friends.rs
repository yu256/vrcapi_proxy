use crate::get_img;
use crate::global::{FAVORITE_FRIENDS, FRIENDS, INVALID_AUTH};
use crate::websocket::User;
use anyhow::{Context as _, Result};
use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Serialize)]
struct Friend {
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
            currentAvatarThumbnailImageUrl: get_img!(user, clone),
            id: user.id.to_owned(),
            status: user.status.to_owned(),
            location: user.location.to_owned(),
            undetermined: user.undetermined,
        }
    }
}

pub(crate) async fn api_friends(req: String) -> Result<ResFriend> {
    let (public, private) = FRIENDS
        .read()
        .await
        .get(&req)
        .context(INVALID_AUTH)?
        .iter()
        .map(Friend::from)
        .partition(|friend| friend.location != "private");

    Ok(ResFriend { public, private })
}

pub(crate) async fn api_friends_filtered(req: String) -> Result<ResFriend> {
    let unlocked = FAVORITE_FRIENDS.read().await;
    let favorites = unlocked.get(&req).context(INVALID_AUTH)?;
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
