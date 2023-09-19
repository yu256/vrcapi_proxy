use super::structs::FriendOnlineEventContent;
use crate::api::User;

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
