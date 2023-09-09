use crate::{
    api::FRIENDS,
    consts::{UA, UA_VALUE},
    websocket::structs::{
        FriendOnlineEventContent, FriendUpdateEventContent, StreamBody, UserIdContent,
    },
};
use anyhow::{bail, Context as _, Result};
use futures::StreamExt;
use rocket::tokio::{self, time::sleep};
use std::sync::{
    atomic::{AtomicU8, Ordering},
    Arc,
};
use tokio_tungstenite::{connect_async, tungstenite::client::IntoClientRequest};

pub(crate) async fn stream(data: Arc<(String, String)>) -> Result<()> {
    let mut req = format!("wss://pipeline.vrchat.cloud/?{}", &data.1).into_client_request()?;
    let headers = req.headers_mut();
    headers.insert(UA, UA_VALUE.try_into()?);

    // 0: Pingなし, 1: 正常, 2: トークン切れ
    let count = Arc::new(AtomicU8::new(0));
    let cloned_count = Arc::clone(&count);

    let handle = tokio::spawn(async move {
        let (stream, _) = connect_async(req).await?;

        let (_, mut read) = stream.split();

        while let Some(message) = read.next().await {
            let data = Arc::clone(&data);
            let cloned_count = Arc::clone(&cloned_count);
            tokio::spawn(async move {
                let msg = message?;
                let message = msg.to_string();

                if message.contains("authToken doesn't correspond with an active session") {
                    cloned_count.store(2, Ordering::Release);
                    return Ok::<(), anyhow::Error>(());
                }

                if msg.is_ping() {
                    cloned_count.store(1, Ordering::Release);
                } else if let Ok(body) = serde_json::from_str::<StreamBody>(&message) {
                    match body.r#type.as_str() {
                        "friend-online" | "friend-location" => {
                            if let Ok(content) =
                                serde_json::from_str::<FriendOnlineEventContent>(&body.content)
                            {
                                let mut unlocked = FRIENDS.write().await;
                                let friends =
                                    unlocked.get_mut(&data.0).context("No friends found.")?;
                                if let Some(friend) = friends
                                    .iter_mut()
                                    .find(|friend| friend.id == content.user.id)
                                {
                                    *friend = content.into();
                                } else {
                                    friends.push(content.into());
                                }
                            } else {
                                eprintln!("not deserialized: {message}"); // debug
                            }
                        }
                        "friend-add" | "friend-update" => {
                            if let Ok(content) =
                                serde_json::from_str::<FriendUpdateEventContent>(&body.content)
                            {
                                let mut unlocked = FRIENDS.write().await;
                                let friends =
                                    unlocked.get_mut(&data.0).context("No friends found.")?;
                                if let Some(friend) = friends
                                    .iter_mut()
                                    .find(|friend| friend.id == content.user.id)
                                {
                                    *friend = content.user;
                                } else {
                                    friends.push(content.user);
                                }
                            } else {
                                eprintln!("not deserialized: {message}"); // debug
                            }
                        }
                        "friend-offline" | "friend-delete" | "friend-active" => {
                            if let Ok(content) =
                                serde_json::from_str::<UserIdContent>(&body.content)
                            {
                                let mut unlocked = FRIENDS.write().await;
                                let friends =
                                    unlocked.get_mut(&data.0).context("No friends found.")?;
                                friends.retain(|f| f.id != content.userId)
                            } else {
                                eprintln!("not deserialized: {message}"); // debug
                            }
                        }
                        _ => {}
                    }
                }
                Ok(())
            });
        }
        Ok::<(), anyhow::Error>(())
    });

    loop {
        sleep(std::time::Duration::from_secs(60)).await;
        match count.fetch_add(0, Ordering::Acquire) {
            1 => {}
            0 => {
                if handle.is_finished() {
                    return handle.await?;
                } else {
                    handle.abort();
                    bail!("disconnected.");
                }
            }
            _ => return Ok(()),
        }
    }
}
