use super::{request, utils::find_matched_data};
use crate::global::FAVORITE_FRIENDS;
use crate::{api::utils::request_json, split_colon};
use anyhow::Result;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct Favorite {
    // id: String,
    // r#type: String,
    favoriteId: String,
    // tags: Vec<String>,
}

pub(crate) async fn api_add_favorites(req: String) -> Result<bool> {
    split_colon!(req, [auth, r#type, id, tag]);

    let token = find_matched_data(auth)?.1;

    request_json(
        "POST",
        "https://api.vrchat.cloud/api/1/favorites",
        &token,
        json!( {"type": r#type, "favoriteId": id, "tags": [tag]} ),
    )
    .map(|_| true)
}

pub(crate) async fn api_re_fetch(req: String) -> Result<bool> {
    let token = find_matched_data(&req)?.1;
    fetch_favorite_friends(req, &token).await.map(|_| true)
}

pub(crate) async fn fetch_favorite_friends(auth: String, token: &str) -> Result<()> {
    FAVORITE_FRIENDS.write().await.insert(
        auth,
        request(
            "GET",
            "https://api.vrchat.cloud/api/1/favorites?type=friend",
            token,
        )?
        .into_json::<Vec<Favorite>>()?
        .into_iter()
        .map(|favorite| favorite.favoriteId)
        .collect(),
    );

    Ok(())
}
