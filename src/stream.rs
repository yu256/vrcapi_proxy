use crate::{
    api::{User, FRIENDS},
    consts::{UA, UA_VALUE},
};
use anyhow::{Context as _, Result};
use futures::StreamExt;
use serde::Deserialize;
use tokio_tungstenite::{connect_async, tungstenite::client::IntoClientRequest};

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct FriendOnlineEvent {
    r#type: String,
    content: FriendOnlineEventContent,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct FriendOnlineEventContent {
    user: User,
}

pub(crate) async fn stream(data: &(String, String)) -> Result<()> {
    let mut req = format!("wss://pipeline.vrchat.cloud/?{}", &data.1).into_client_request()?;
    let headers = req.headers_mut();
    headers.insert(UA, UA_VALUE.try_into()?);

    let (stream, _) = connect_async(req).await?;

    let (_, mut read) = stream.split();

    while let Some(message) = read.next().await {
        println!("{message:?}"); // debug
        if let Ok(body) = serde_json::from_str::<FriendOnlineEvent>(&message?.to_string()) {
            let mut unlocked = FRIENDS.write().await;
            let friends = unlocked.get_mut(&data.0).context("No friends found.")?;
            match body.r#type.as_str() {
                "friend-offline" => friends.retain(|f| f.id != body.content.user.id),
                "friend-online" | "friend-location" | "friend-update" => {
                    if let Some(friend) = friends
                        .iter_mut()
                        .find(|friend| friend.id == body.content.user.id)
                    {
                        *friend = body.content.user;
                    } else {
                        friends.push(body.content.user);
                    }
                }
                "friend-add" => {
                    if body.content.user.location != "offline" {
                        friends.push(body.content.user);
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}
