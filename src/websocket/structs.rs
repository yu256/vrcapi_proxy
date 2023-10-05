use crate::unsanitizer::Unsanitizer;
use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize, Clone)]
pub(crate) struct User {
    #[serde(default)]
    pub(crate) bio: String,
    #[serde(default)]
    pub(crate) bioLinks: Vec<String>,
    #[serde(default)]
    pub(crate) currentAvatarThumbnailImageUrl: String,
    pub(crate) displayName: String,
    pub(crate) id: String,
    pub(crate) isFriend: bool,
    pub(crate) location: String,
    pub(crate) travelingToLocation: Option<String>,
    pub(crate) status: String,
    #[serde(default)]
    pub(crate) statusDescription: String,
    pub(crate) tags: Vec<String>,
    #[serde(default)]
    pub(crate) userIcon: String,
    #[serde(default)]
    pub(crate) profilePicOverride: String,
    #[serde(default)]
    pub(crate) undetermined: bool,
}

impl User {
    pub(crate) fn unsanitize(&mut self) {
        self.bio = self.bio.unsanitize();
        self.statusDescription = self.statusDescription.unsanitize();
    }
}

pub(crate) trait VecUserExt {
    fn update(&mut self, content: impl Into<User>);
    fn del(&mut self, id: &str);
    fn unsanitize(&mut self);
}

impl VecUserExt for Vec<User> {
    fn update(&mut self, content: impl Into<User>) {
        let mut user = content.into();
        user.unsanitize();
        if let Some(friend) = self.iter_mut().find(|friend| friend.id == user.id) {
            *friend = user;
        } else {
            self.push(user);
        }
    }
    fn del(&mut self, id: &str) {
        if let Some(index) = self.iter().position(|x| x.id == id) {
            self.remove(index);
        }
    }
    fn unsanitize(&mut self) {
        self.iter_mut().for_each(User::unsanitize);
    }
}

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
    pub(crate) travelingToLocation: Option<String>,
    pub(crate) user: OnlineEventUser,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub(crate) struct OnlineEventUser {
    #[serde(default)]
    pub(crate) bio: String,
    #[serde(default)]
    pub(crate) bioLinks: Vec<String>,
    #[serde(default)]
    pub(crate) currentAvatarThumbnailImageUrl: String,
    pub(crate) displayName: String,
    pub(crate) id: String,
    pub(crate) isFriend: bool,
    pub(crate) status: String,
    #[serde(default)]
    pub(crate) statusDescription: String,
    pub(crate) tags: Vec<String>,
    #[serde(default)]
    pub(crate) userIcon: String,
    #[serde(default)]
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
