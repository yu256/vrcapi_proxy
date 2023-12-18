use crate::user_impl::{Status, User};
use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub(super) struct StreamBody {
    pub(super) r#type: String,
    pub(super) content: String, // json
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub(super) struct FriendOnlineEventContent {
    pub(super) location: String,
    pub(super) travelingToLocation: Option<String>,
    pub(super) user: OnlineEventUser,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub(super) struct OnlineEventUser {
    #[serde(default)]
    pub(super) bio: String,
    #[serde(default)]
    pub(super) bioLinks: Vec<String>,
    #[serde(default)]
    pub(super) currentAvatarThumbnailImageUrl: String,
    pub(super) displayName: String,
    pub(super) id: String,
    pub(super) isFriend: bool,
    pub(super) status: Status,
    #[serde(default)]
    pub(super) statusDescription: String,
    pub(super) tags: Vec<String>,
    #[serde(default)]
    pub(super) userIcon: String,
    #[serde(default)]
    pub(super) profilePicOverride: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub(super) struct FriendUpdateEventContent {
    pub(super) user: User,
}

impl From<FriendOnlineEventContent> for User {
    fn from(value: FriendOnlineEventContent) -> Self {
        Self {
            bio: value.user.bio,
            bioLinks: value.user.bioLinks,
            currentAvatarThumbnailImageUrl: value.user.currentAvatarThumbnailImageUrl,
            displayName: value.user.displayName,
            id: value.user.id,
            isFriend: value.user.isFriend,
            location: value.location,
            travelingToLocation: value.travelingToLocation,
            status: value.user.status,
            statusDescription: value.user.statusDescription,
            tags: value.user.tags,
            userIcon: value.user.userIcon,
            profilePicOverride: value.user.profilePicOverride,
            undetermined: false,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub(super) struct UserIdContent {
    pub(super) userId: String,
}
