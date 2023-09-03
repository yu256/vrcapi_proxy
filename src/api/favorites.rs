use super::utils::{find_matched_data, CLIENT};
use crate::{
    api::response::ApiResponse,
    consts::{COOKIE, UA, UA_VALUE},
    into_err, split_colon,
};
use anyhow::{bail, Result};
use rocket::{http::Status, serde::json::Json};
use serde_json::json;
const URL: &str = "https://api.vrchat.cloud/api/1/favorites";

#[post("/favorites", data = "<req>")]
pub(crate) async fn api_add_favorites(req: &str) -> (Status, Json<ApiResponse<bool>>) {
    match fetch(req).await {
        Ok(_) => (Status::Ok, Json(true.into())),

        Err(error) => (Status::InternalServerError, Json(into_err!(error))),
    }
}

async fn fetch(req: &str) -> Result<()> {
    split_colon!(req, [auth, r#type, id, tag]);

    let (_, token) = find_matched_data(auth)?;

    let res = CLIENT
        .post(URL)
        .set(UA, UA_VALUE)
        .set(COOKIE, &token)
        .send_json(json!( {"type": r#type, "favoriteId": id, "tags": [tag]} ))?;

    if res.status() == 200 {
        Ok(())
    } else if res.status() == 400 {
        bail!("既に登録されています。")
    } else {
        bail!("{}", res.into_string()?)
    }
}
