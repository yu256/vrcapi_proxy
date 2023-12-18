#![feature(lazy_cell)]
#![feature(let_chains)]

use crate::init::init;
use anyhow::Result;
use api::*;
use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderValue, Method};
use axum::{routing::post, Router};
use fetch_friends::spawn;
use tower_http::cors::CorsLayer;

mod api;
mod fetch_friends;
mod general;
mod global;
mod init;
mod macros;
mod unsanitizer;
mod user_impl;
mod websocket;

#[tokio::main]
async fn main() -> Result<()> {
    init()?;

    spawn().await;

    let (cors, listen) = {
        let data = general::read_json::<init::Data>("data.json")?;
        (data.cors.parse::<HeaderValue>()?, data.listen.parse()?)
    };

    let app = Router::new()
        .route(
            "/reboot",
            post(move |req: String| async move {
                validate!(req);
                spawn().await;
                Ok(true)
            }),
        )
        .route("/auth", post(api_auth))
        .route("/user", post(api_user))
        .route("/profile", post(api_update_profile))
        .route("/friends", post(api_friends))
        .route("/friends/filtered", post(api_friends_filtered))
        .route("/friend/request", post(api_friend_request))
        .route("/friend/accept", post(api_friend_accept))
        .route("/friend/status", post(api_friend_status))
        .route("/invite/myself", post(api_invite_myself))
        .route("/notifications", post(api_notifications))
        .route("/search/user", post(api_search_user))
        .route("/twofactor", post(api_twofactor))
        .route("/favorites", post(api_add_favorites))
        .route("/favorites/refresh", post(api_re_fetch))
        .route("/group", post(api_group))
        .route("/instance", post(api_instance))
        .route("/world", post(api_world))
        .layer(
            CorsLayer::new()
                .allow_origin(cors)
                .allow_methods([Method::POST])
                .allow_headers([CONTENT_TYPE]),
        );

    axum::Server::bind(&listen)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
