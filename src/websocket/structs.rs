use crate::unsanitizer::Unsanitizer;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Clone, Eq)]
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
    pub(crate) status: Status,
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

#[derive(Serialize, Deserialize, Ord, PartialEq, PartialOrd, Eq, Clone, Copy)]
pub(crate) enum Status {
    #[serde(rename = "join me")]
    JoinMe,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "ask me")]
    AskMe,
    #[serde(rename = "busy")]
    Busy,
}

impl User {
    pub(crate) fn unsanitize(&mut self) {
        self.bio = self.bio.unsanitize();
        self.statusDescription = self.statusDescription.unsanitize();
    }
}

impl Ord for User {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.status.cmp(&other.status)
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status
    }
}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.status.cmp(&other.status))
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
        self.sort();
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
    pub(crate) status: Status,
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
