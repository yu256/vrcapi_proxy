use super::error::WSError;
use crate::global::{AUTHORIZATION, FRIENDS, USERS};
use crate::user_impl::{Status, User, VecUserExt as _};
use crate::{
    api::request,
    global::{APP_NAME, UA},
    websocket::structs::{
        FriendOnlineEventContent, FriendUpdateEventContent, StreamBody, UserIdContent,
    },
};
use futures::StreamExt as _;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Release};
use tokio_tungstenite::tungstenite::http::HeaderValue;
use tokio_tungstenite::{connect_async, tungstenite::client::IntoClientRequest as _};
use trie_match::trie_match;
use WSError::*;

static IS_DISCONNECTED: AtomicBool = AtomicBool::new(false);

pub(crate) async fn stream() -> WSError {
	// Safety: トークンがあっているなら失敗するはずがない 不正であればこの関数に到達しない
    let mut req = unsafe {
        format!(
            "wss://pipeline.vrchat.cloud/?{}",
            &*AUTHORIZATION.1.read().await
        )
        .into_client_request()
        .unwrap_unchecked()
    };

    req.headers_mut()
        .insert(UA, HeaderValue::from_static(APP_NAME));

    IS_DISCONNECTED.store(true, Release);

    let handle = tokio::spawn(async {
        let (stream, _) = match connect_async(req).await {
            Ok(ok) => ok,
            Err(e) => {
                use tokio_tungstenite::tungstenite::error::Error::Io;
                return if let Io(io_err) = e {
                    OtherErr(io_err.to_string())
                } else {
                    eprintln!("Unknown Error: {e}");
                    UnknownErr
                };
            }
        };

        let (_, mut read) = stream.split();

        while let Some(message) = read.next().await {
            let message = match message {
                Ok(message) if message.is_ping() => {
                    IS_DISCONNECTED.store(false, Release);
                    continue;
                }
                Ok(message) => message.to_string(),
                Err(e) => return OtherErr(e.to_string()),
            };

            if message.starts_with(r#"{"err"#) {
                if !message.contains("authToken") {
                    eprintln!("Unknown Error: {message}");
                }
                return TokenErr;
            }

            tokio::spawn(async move {
                let body = serde_json::from_str::<StreamBody>(&message)?;
                trie_match! {
                    match body.r#type.as_str() {
                        "friend-online" | "friend-location" => {
                            let content = serde_json::from_str::<FriendOnlineEventContent>(&body.content)?;
                            FRIENDS.write(|friends| friends.update(content)).await;
                        }

                        "friend-update" => {
                            let user =
                                serde_json::from_str::<FriendUpdateEventContent>(&body.content)?.user;
                            FRIENDS.write(|friends| friends.update(user)).await;
                        }

                        "friend-add" => {
                            let id = serde_json::from_str::<UserIdContent>(&body.content)?.userId;
                            let mut new_friend = request(
                                "GET",
                                &format!("https://api.vrchat.cloud/api/1/users/{id}"),
                                &AUTHORIZATION.1.read().await,
                            )?
                            .into_json::<User>()?;

                            if new_friend.location != "offline" {
                                if let Status::AskMe | Status::Busy = new_friend.status {
                                    new_friend.undetermined = true;
                                }
                                FRIENDS.write(|friends| friends.update(new_friend)).await;
                            }
                        }

                        "friend-offline" | "friend-delete" | "friend-active" => {
                            let id = serde_json::from_str::<UserIdContent>(&body.content)?.userId;
                            FRIENDS.write(|friends| friends.del(&id)).await;
                        }

                        "user-update" => {
                            let user = serde_json::from_str::<FriendUpdateEventContent>(&body.content)?.user;
                            USERS.insert(user).await;
                        }

                        "user-location" => {
                            let user = serde_json::from_str::<FriendOnlineEventContent>(&body.content)?.into();
                            USERS.insert(user).await;
                        }
                        _ => {}
                    }
                }

                Ok::<(), anyhow::Error>(())
            });
        }
        OtherErr("disconnected".into())
    });

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        {
            if IS_DISCONNECTED.fetch_xor(true, Acquire) {
                return if handle.is_finished() {
                    match handle.await {
                        Ok(err) => err,
                        Err(e) => OtherErr(e.to_string()),
                    }
                } else {
                    handle.abort();
                    OtherErr("disconnected".into())
                };
            }
            IS_DISCONNECTED.store(true, Release);
        }
    }
}
