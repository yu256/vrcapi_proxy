use crate::{
    api::FRIENDS,
    consts::{UA, UA_VALUE},
    websocket::structs::{
        FriendOnlineEventContent, FriendUpdateEventContent, StreamBody, UserIdContent,
    },
};
use anyhow::{anyhow, Context as _, Result};
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
            if let Ok(body) = serde_json::from_str::<StreamBody>(&message) {
                match body.r#type.as_str() {
                    "friend-online" | "friend-location" => {
                        if let Ok(content) =
                            serde_json::from_str::<FriendOnlineEventContent>(&body.content)
                        {
                            let mut unlocked = FRIENDS.write().await;
                            let friends = unlocked.get_mut(&data.0).context("No friends found.")?;
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
                            let friends = unlocked.get_mut(&data.0).context("No friends found.")?;
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
                        if let Ok(content) = serde_json::from_str::<UserIdContent>(&body.content) {
                            let mut unlocked = FRIENDS.write().await;
                            let friends = unlocked.get_mut(&data.0).context("No friends found.")?;
                            friends.retain(|f| f.id != content.userId)
                        } else {
                            eprintln!("not deserialized: {message}"); // debug
                        }
                    }
                    _ => {}
                }
            }
            Ok::<(), anyhow::Error>(())
        });
    }

    Err(anyhow!("disconnected"))
}
