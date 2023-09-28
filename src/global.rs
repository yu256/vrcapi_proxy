use crate::websocket::User;
use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};
use tokio::sync::RwLock;

pub(crate) const UA: &str = "User-Agent";
pub(crate) const UA_VALUE: &str = "vrcapi-proxy";
pub(crate) const COOKIE: &str = "Cookie";
pub(crate) const INVALID_AUTH: &str = "サーバー側の初回fetchに失敗しているか、トークンが無効です。";

pub(crate) static FRIENDS: LazyLock<RwLock<HashMap<String, Vec<User>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));
pub(crate) static FAVORITE_FRIENDS: LazyLock<RwLock<HashMap<String, HashSet<String>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));
