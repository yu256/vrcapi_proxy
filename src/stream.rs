use crate::{
    api::{Friend, FRIENDS},
    consts::{UA, UA_VALUE},
    data::Data,
};
use anyhow::{Context as _, Result};
use futures::StreamExt;
use httparse::Header;
use serde::Deserialize;
use tokio_tungstenite::connect_async;

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct FriendOnlineEvent {
    r#type: String,
    content: FriendOnlineEventContent,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct FriendOnlineEventContent {
    user: Friend,
}

pub(crate) async fn stream(data: Data) -> Result<()> {
    let req = format!("wss://pipeline.vrchat.cloud/?{}", &data.token);

    let (stream, _) = connect_async(req).await?;

    let (_, mut read) = stream.split();

    while let Some(message) = read.next().await {
        if let Ok(body) = serde_json::from_str::<FriendOnlineEvent>(&message?.to_string()) {
            let mut unlocked = FRIENDS.lock().await;
            let friends = unlocked.get_mut(&data.auth).context("No friends found.")?;
            match body.r#type.as_str() {
                "friend-offline" => friends.retain(|f| f.id != body.content.user.id),
                "friend-online" | "friend-location" => {
                    if let Some(friend) = friends
                        .iter_mut()
                        .find(|friend| friend.id == body.content.user.id)
                    {
                        *friend = body.content.user;
                    } else {
                        friends.insert(0, body.content.user);
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}
