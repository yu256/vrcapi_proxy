use super::struct_impl::{OnlineFriends, Users};
use dirs_2::home_dir;
use std::{collections::HashSet, path::PathBuf, sync::LazyLock};
use tokio::sync::RwLock;

pub(crate) const APP_NAME: &str = "vrcapi_proxy";
pub(crate) const UA: &str = "User-Agent";
pub(crate) const COOKIE: &str = "Cookie";
pub(crate) const INVALID_AUTH: &str = "認証情報が不正です。";
pub(crate) const INVALID_REQUEST: &str = "リクエストが不正です。";

pub(crate) static DATA_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    home_dir()
        .expect("ホームディレクトリの取得に失敗しました。")
        .join(APP_NAME)
});

// Interior mutable
pub(crate) static FRIENDS: OnlineFriends = OnlineFriends {
    inner: LazyLock::new(|| RwLock::new(Vec::new())),
};

pub(crate) static USERS: LazyLock<Users> = LazyLock::new(|| Users {
    inner: RwLock::new(None),
});

pub(crate) static FAVORITE_FRIENDS: LazyLock<RwLock<HashSet<String>>> =
    LazyLock::new(|| RwLock::new(HashSet::new()));

pub(crate) static HANDLER: LazyLock<RwLock<Option<tokio::task::JoinHandle<()>>>> =
    LazyLock::new(|| RwLock::new(None));

pub(crate) static AUTHORIZATION: LazyLock<(&'static str, RwLock<String>)> = LazyLock::new(|| {
    let data = crate::general::read_json::<crate::init::Data>("data.json").unwrap();
    (data.auth.leak(), RwLock::new(data.token))
});
