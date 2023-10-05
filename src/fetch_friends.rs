use crate::global::{COLOR, FRIENDS};
use crate::websocket::structs::VecUserExt as _;
use crate::websocket::User;
use crate::{
    api::{fetch_favorite_friends, request},
    websocket::stream::stream,
};
use std::sync::atomic::Ordering;
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

        let color = COLOR.fetch_add(1, Ordering::Relaxed);

        println!(
            "\x1b[38;5;{}mTrying to connect stream... ({})\x1b[m",
            color, &data.0
        );

        match fetch_friends(&data.1) {
            Ok(mut friends) => {
                let _ = fetch_favorite_friends(data.0.clone(), &data.1).await;

                friends.retain_mut(|friend| {
                    if friend.location == "offline" {
                        false
                    } else {
                        if friend.status == "ask me" || friend.status == "busy" {
                            friend.undetermined = true;
                        }
                        true
                    }
                });

                friends.unsanitize();

                FRIENDS.write().await.insert(data.0.clone(), friends);

                loop {
                    if stream(Arc::clone(&data)).await.is_ok() {
                        FRIENDS.write().await.remove(&data.0);
                        println!(
                            "\x1b[38;5;{}mトークンが失効しました。 ({})\x1b[m",
                            color, &data.0
                        );
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("\x1b[38;5;{}mError: {}\x1b[m", color, e);
            }
        }
    });
}
