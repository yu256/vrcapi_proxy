use crate::{
    api::{fetch_favorite_friends, request, User, FRIENDS},
    websocket::stream::stream,
};
use rocket::tokio;
use std::sync::Arc;

pub(crate) fn fetch_friends(token: &str) -> anyhow::Result<Vec<User>> {
    request(
        "GET",
        "https://api.vrchat.cloud/api/1/auth/user/friends?offline=false",
        token,
    )?
    .into_json()
    .map_err(From::from)
}

pub(crate) fn spawn(data: (String, String)) {
    tokio::spawn(async move {
        let data = Arc::new(data);

        match fetch_friends(&data.1) {
            Ok(mut friends) => {
                let _ = fetch_favorite_friends(&data.0, &data.1).await;

                friends.retain_mut(|friend| {
                    if friend.location == "offline" {
                        false
                    } else {
                        if friend.status == "ask me" {
                            friend.undetermined = true;
                        }
                        true
                    }
                });

                FRIENDS.write().await.insert(data.0.clone(), friends);

                loop {
                    if stream(Arc::clone(&data)).await.is_ok() {
                        FRIENDS.write().await.remove(&data.0);
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error(fetch_friends()): {}", e);
            }
        }
    });
}
