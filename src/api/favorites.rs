use super::utils::{StrExt as _, CLIENT};
use crate::{
    consts::{COOKIE, UA, UA_VALUE},
    general::find_matched_data,
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
    let (auth, r#type, id, tag) = {
        let (auth, rest) = req.split_colon()?;
        let (r#type, rest) = rest.split_colon()?;
        let (id, tag) = rest.split_colon()?;

        (auth, r#type, id, tag)
    };

    let matched = find_matched_data(auth)?;

    let res = CLIENT
        .post(URL)
        .header(UA, UA_VALUE)
        .header(COOKIE, &matched.token)
        .json(&json!( {"type": r#type, "favoriteId": id, "tags": [tag]} ))
        .send()
        .await?;

    if res.status().is_success() {
        Ok(())
    } else if res.status() == 400 {
        bail!("既に登録されています。")
    } else {
        bail!("{}", res.status())
    }
}
