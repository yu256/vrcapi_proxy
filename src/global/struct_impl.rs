use crate::user_impl::User;
use std::sync::LazyLock;
use tokio::sync::RwLock;

pub(crate) struct OnlineFriends {
    pub(super) inner: LazyLock<RwLock<Vec<User>>>,
}

impl OnlineFriends {
    pub(crate) async fn read<T, F>(&self, fun: F) -> T
    where
        F: FnOnce(&Vec<User>) -> T,
    {
        let friends = self.inner.read().await;
        fun(&friends)
    }

    pub(crate) async fn write<F>(&self, fun: F)
    where
        F: FnOnce(&mut Vec<User>),
    {
        let mut friends = self.inner.write().await;
        fun(&mut friends)
    }
}

pub(crate) struct Users {
    pub(super) inner: RwLock<Option<User>>,
}

impl Users {
    pub(crate) async fn insert(&self, user: User) {
        *self.inner.write().await = Some(user);
    }
    pub(crate) async fn read(&self) -> Option<User> {
        self.inner.read().await.clone()
    }
    pub(crate) async fn write(&self, fun: impl FnOnce(&mut User)) {
        if let Some(ref mut user) = *self.inner.write().await {
            fun(user)
        }
    }
}
