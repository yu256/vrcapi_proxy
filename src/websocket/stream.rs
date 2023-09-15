use crate::{
    api::{request, User, FRIENDS},
    consts::{UA, UA_VALUE},
    websocket::structs::{
        FriendOnlineEventContent, FriendUpdateEventContent, StreamBody, UserIdContent,
    },
};
use anyhow::{anyhow, Result};
use futures::StreamExt;
use rocket::tokio;
use std::sync::Arc;
use tokio_tungstenite::{connect_async, tungstenite::client::IntoClientRequest};

pub(crate) async fn stream(data: Arc<(String, String)>) -> Result<()> {
    let mut req = format!("wss://pipeline.vrchat.cloud/?{}", &data.1).into_client_request()?;
    let headers = req.headers_mut();
    headers.insert(UA, UA_VALUE.try_into()?);

    let (stream, _) = connect_async(req).await?;

    let (_, mut read) = stream.split();

    while let Some(message) = read.next().await {
        let message = message?.to_string();
        if message.starts_with(r#"{"err":"authToken"#) {
            return Ok(());
        }
        let data = Arc::clone(&data);

        tokio::spawn(async move {
            let body = serde_json::from_str::<StreamBody>(&message)?;
            match body.r#type.as_str() {
                "friend-online" | "friend-location" => {
                    if let Ok(content) =
                        serde_json::from_str::<FriendOnlineEventContent>(&body.content)
                    {
                        let mut unlocked = FRIENDS.write().await;
                        let Some(friends) = unlocked.get_mut(&data.0) else {
                            return Ok(());
                        };
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

                "friend-update" => {
                    if let Ok(content) =
                        serde_json::from_str::<FriendUpdateEventContent>(&body.content)
                    {
                        let mut unlocked = FRIENDS.write().await;
                        let Some(friends) = unlocked.get_mut(&data.0) else {
                            return Ok(());
                        };
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

                "friend-add" => {
                    let content = serde_json::from_str::<UserIdContent>(&body.content)?;
                    let mut new_friend = request(
                        "GET",
                        &format!("https://api.vrchat.cloud/api/1/users/{}", content.userId),
                        &data.1,
                    )?
                    .into_json::<User>()?;

                    if new_friend.location != "offline" {
                        if new_friend.status == "ask me" {
                            new_friend.undetermined = true;
                        }
                        let mut unlocked = FRIENDS.write().await;
                        let Some(friends) = unlocked.get_mut(&data.0) else {
                            return Ok(());
                        };
                        friends.push(new_friend);
                    }
                }

                "friend-offline" | "friend-delete" | "friend-active" => {
                    if let Ok(content) = serde_json::from_str::<UserIdContent>(&body.content) {
                        let mut unlocked = FRIENDS.write().await;
                        let Some(friends) = unlocked.get_mut(&data.0) else {
                            return Ok(());
                        };
                        friends.retain(|f| f.id != content.userId)
                    } else {
                        eprintln!("not deserialized: {message}"); // debug
                    }
                }
                _ => {}
            }

            Ok::<(), anyhow::Error>(())
        });
    }

    Err(anyhow!("disconnected"))
}
