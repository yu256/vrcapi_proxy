use crate::websocket::User;
use anyhow::{anyhow, Result};
use std::sync::atomic::AtomicU8;
use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};
use tokio::sync::RwLock;

pub(crate) const UA: &str = "User-Agent";
pub(crate) const UA_VALUE: &str = "vrcapi-proxy";
pub(crate) const COOKIE: &str = "Cookie";
pub(crate) const INVALID_AUTH: &str = "サーバー側の初回fetchに失敗しているか、トークンが無効です。";

pub(crate) static FRIENDS: OnlineFriends = OnlineFriends {
    inner: LazyLock::new(|| RwLock::new(HashMap::new())),
};

pub(crate) static USERS: LazyLock<Users> = LazyLock::new(|| Users {
    inner: RwLock::new(HashMap::new()),
});

pub(crate) static FAVORITE_FRIENDS: LazyLock<RwLock<HashMap<String, HashSet<String>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub(crate) static COLOR: AtomicU8 = AtomicU8::new(1);

pub(crate) struct OnlineFriends {
    inner: LazyLock<RwLock<HashMap<String, Vec<User>>>>,
}

impl OnlineFriends {
    pub(crate) async fn read<T, F>(&self, auth: &str, fun: F) -> Result<T>
    where
        F: FnOnce(&Vec<User>) -> T,
    {
        let friends = self.inner.read().await;
        if let Some(friends) = friends.get(auth) {
            Ok(fun(friends))
        } else {
            Err(anyhow!(INVALID_AUTH))
        }
    }

    pub(crate) async fn write<F>(&self, auth: &str, fun: F)
    where
        F: FnOnce(&mut Vec<User>),
    {
        let mut friends = self.inner.write().await;
        if let Some(friends) = friends.get_mut(auth) {
            fun(friends)
        }
    }

    pub(crate) async fn insert(&self, key: String, value: Vec<User>) {
        self.inner.write().await.insert(key, value);
    }

    pub(crate) async fn remove(&self, key: &str) {
        self.inner.write().await.remove(key);
    }
}

pub(crate) struct Users {
    inner: RwLock<HashMap<String, User>>,
}

impl Users {
    pub(crate) async fn insert(&self, auth: &str, user: User) {
        self.inner.write().await.insert(auth.to_owned(), user);
    }
    pub(crate) async fn read(&self, auth: &str) -> Option<User> {
        self.inner.read().await.get(auth).cloned()
    }
    pub(crate) async fn write(&self, auth: &str, fun: impl FnOnce(&mut User)) {
        let mut users = self.inner.write().await;
        if let Some(user) = users.get_mut(auth) {
            fun(user)
        }
    }
}
