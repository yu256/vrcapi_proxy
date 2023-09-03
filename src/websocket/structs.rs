use crate::api::User;
use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub(crate) struct StreamBody {
    pub(crate) r#type: String,
    pub(crate) content: String, // json
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub(crate) struct FriendOnlineEventContent {
    pub(crate) location: String,
    pub(crate) user: OnlineEventUser,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub(crate) struct OnlineEventUser {
    pub(crate) bio: String,
    pub(crate) bioLinks: Vec<String>,
    pub(crate) currentAvatarThumbnailImageUrl: String,
    pub(crate) displayName: String,
    pub(crate) id: String,
    pub(crate) isFriend: bool,
    pub(crate) status: String,
    pub(crate) statusDescription: String,
    pub(crate) tags: Vec<String>,
    pub(crate) userIcon: String,
    pub(crate) profilePicOverride: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub(crate) struct FriendUpdateEventContent {
    pub(crate) user: User,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub(crate) struct UserIdContent {
    pub(crate) userId: String,
}
