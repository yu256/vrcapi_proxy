use crate::{
    api::{fetch_friends, FRIENDS},
    websocket::stream::stream,
};
use rocket::tokio;
use std::sync::Arc;

pub(crate) fn spawn(data: (String, String)) {
    tokio::spawn(async move {
        let data = Arc::new(data);
        match fetch_friends(&data.1) {
            Ok(friends) => {
                let friends = friends
                    .into_iter()
                    .filter_map(|mut friend| {
                        if friend.location == "offline" {
                            None
                        } else {
                            if friend.status == "ask me" {
                                friend.undetermined = true;
                            }
                            Some(friend)
                        }
                    })
                    .collect();
                FRIENDS.write().await.insert(data.0.clone(), friends);

                loop {
                    if let Err(e) = stream(Arc::clone(&data)).await {
                        if e.to_string() == "invalid Auth" {
                            FRIENDS.write().await.remove(&data.0);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error(fetch_friends()): {}", e);
            }
        }
    });
}
