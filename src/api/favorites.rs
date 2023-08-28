use super::utils::{find_matched_data, CLIENT};
use crate::{
    consts::{COOKIE, UA, UA_VALUE},
    split_colon,
};
use anyhow::{bail, Result};
use rocket::{http::Status, serde::json::Json};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub(crate) enum Response {
    Success(bool),
    Error(String),
}

const URL: &str = "https://api.vrchat.cloud/api/1/favorites";

#[post("/favorites", data = "<req>")]
pub(crate) async fn api_add_favorites(req: &str) -> (Status, Json<Response>) {
    match fetch(req).await {
        Ok(_) => (Status::Ok, Json(Response::Success(true))),

        Err(error) => (
            Status::InternalServerError,
            Json(Response::Error(error.to_string())),
        ),
    }
}

async fn fetch(req: &str) -> Result<()> {
    split_colon!(req, [auth, r#type, id, tag]);

    let (_, token) = find_matched_data(auth)?;

    let res = CLIENT
        .post(URL)
        .header(UA, UA_VALUE)
        .header(COOKIE, &token)
        .json(&json!( {"type": r#type, "favoriteId": id, "tags": [tag]} ))
        .send()
        .await?;

    if res.status().is_success() {
        Ok(())
    } else if res.status() == 400 {
        bail!("既に登録されています。")
    } else {
        bail!("{}", res.text().await?)
    }
}
