use crate::global::{FRIENDS, USERS};
use crate::websocket::structs::{Status, VecUserExt as _};
use crate::websocket::User;
use crate::{
    api::request,
    global::{UA, UA_VALUE},
    websocket::structs::{
        FriendOnlineEventContent, FriendUpdateEventContent, StreamBody, UserIdContent,
    },
};
use anyhow::{anyhow, Result};
use futures::StreamExt as _;
use std::sync::Arc;
use tokio_tungstenite::{connect_async, tungstenite::client::IntoClientRequest as _};
use trie_match::trie_match;

pub(crate) async fn stream(data: Arc<(String, String)>) -> Result<()> {
    let mut req = format!("wss://pipeline.vrchat.cloud/?{}", &data.1).into_client_request()?;
    req.headers_mut().insert(UA, UA_VALUE.try_into()?);

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
            trie_match! {
                match body.r#type.as_str() {
                    "friend-online" | "friend-location" => {
                        let content = serde_json::from_str::<FriendOnlineEventContent>(&body.content)?;
                        FRIENDS.write(&data.0, |friends| friends.update(content)).await;
                    }

                    "friend-update" => {
                        let user =
                            serde_json::from_str::<FriendUpdateEventContent>(&body.content)?.user;
                        FRIENDS.write(&data.0, |friends| friends.update(user)).await;
                    }

                    "friend-add" => {
                        let id = serde_json::from_str::<UserIdContent>(&body.content)?.userId;
                        let mut new_friend = request(
                            "GET",
                            &format!("https://api.vrchat.cloud/api/1/users/{id}"),
                            &data.1,
                        )?
                        .into_json::<User>()?;

                        if new_friend.location != "offline" {
                            if let Status::AskMe | Status::Busy = new_friend.status {
                                new_friend.undetermined = true;
                            }
                            FRIENDS.write(&data.0, |friends| friends.update(new_friend)).await;
                        }
                    }

                    "friend-offline" | "friend-delete" | "friend-active" => {
                        let id = serde_json::from_str::<UserIdContent>(&body.content)?.userId;
                        FRIENDS.write(&data.0, |friends| friends.del(&id)).await;
                    }

                    "user-update" => {
                        let user = serde_json::from_str::<FriendUpdateEventContent>(&body.content)?.user;
                        USERS.insert(&data.0, user).await;
                    }

                    "user-location" => {
                        let user = serde_json::from_str::<FriendOnlineEventContent>(&body.content)?.into();
                        USERS.insert(&data.0, user).await;
                    }
                    _ => {}
                }
            }

            Ok::<(), anyhow::Error>(())
        });
    }

    Err(anyhow!("disconnected"))
}
