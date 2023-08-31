use super::utils::{find_matched_data, request};
use crate::split_colon;
use anyhow::{bail, Result};
use rocket::{http::Status, serde::json::Json};
use serde::Serialize;

#[derive(Serialize)]
pub(crate) enum Response {
    Success(bool),
    Error(String),
}

#[post("/friend_request", data = "<req>")]
pub(crate) async fn api_friend_request(req: &str) -> (Status, Json<Response>) {
    match fetch(req).await {
        Ok(_) => (Status::Ok, Json(Response::Success(true))),

        Err(error) => (
            Status::InternalServerError,
            Json(Response::Error(error.to_string())),
        ),
    }
}

async fn fetch(req: &str) -> Result<()> {
    split_colon!(req, [auth, user, method]);

    let (_, token) = find_matched_data(auth)?;

    let res = request(
        method.as_bytes().try_into()?,
        &format!("https://api.vrchat.cloud/api/1/user/{}/friendRequest", user),
        &token,
    )
    .await?;

    if res.status().is_success() {
        Ok(())
    } else {
        bail!("{}", res.text().await?)
    }
}
