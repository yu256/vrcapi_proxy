use crate::general::CustomAndThen as _;
use crate::global::{AUTHORIZATION, FRIENDS, HANDLER};
use crate::user_impl::{Status, User, VecUserExt as _};
use crate::websocket::error::WSError::OtherErr;
use crate::{
    api::{fetch_favorite_friends, request},
    websocket::stream::stream,
};

pub(crate) async fn spawn() {
    if let Some(ref handler) = *HANDLER.read().await {
        if !handler.is_finished() {
            handler.abort();
        }
    }

    *HANDLER.write().await = Some(tokio::spawn(async move {
        println!("Trying to connect stream...");

        let token = &AUTHORIZATION.1.read().await;

        match request(
            "GET",
            "https://api.vrchat.cloud/api/1/auth/user/friends?offline=false",
            token,
        )
        .and_then2(ureq::Response::into_json::<Vec<User>>)
        {
            Ok(mut friends) => {
                let _ = fetch_favorite_friends(token).await;

                friends.retain_mut(|friend| {
                    let is_online = friend.location != "offline";
                    if is_online && let Status::AskMe | Status::Busy = friend.status {
                        friend.undetermined = true;
                    }
                    is_online
                });
                friends.unsanitize();
                friends.sort();

                FRIENDS.write(|users| *users = friends).await;

                while let OtherErr(e) = stream().await {
                    eprintln!("{e}");
                }
            }
            Err(e) => eprintln!("Error: {e}"),
        }
        *HANDLER.write().await = None;
    }));
}
