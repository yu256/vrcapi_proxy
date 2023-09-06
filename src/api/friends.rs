use super::user::User;
use crate::consts::INVALID_AUTH;
use anyhow::Context as _;
use rocket::tokio::sync::RwLock;
use serde::Serialize;
use std::{collections::HashMap, sync::LazyLock};

pub(crate) static FRIENDS: LazyLock<RwLock<HashMap<String, Vec<User>>>> =
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
pub(crate) async fn api_friends(req: &str) -> anyhow::Result<ResFriend> {
    let friends = {
        let read = FRIENDS.read().await;
        read.get(req)
            .context(INVALID_AUTH)?
            .iter()
            .map(Friend::from)
            .collect::<Vec<_>>()
    };

    let mut res = ResFriend {
        public: Vec::new(),
        private: Vec::new(),
    };

    friends.into_iter().for_each(|friend| {
        if friend.location == "private" {
            res.private.push(friend);
        } else {
            res.public.push(friend);
        }
    });

    Ok(res)
}
