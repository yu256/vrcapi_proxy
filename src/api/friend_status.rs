use super::utils::{find_matched_data, request};
use crate::split_colon;
use anyhow::{bail, Result};
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub(crate) struct ResStatus {
    // isFriend: bool,
    outgoingRequest: bool,
    incomingRequest: bool,
}

#[derive(Serialize)]
pub(crate) enum Response {
    Success(ResStatus),
    Error(String),
}

#[post("/friend_status", data = "<req>")]
pub(crate) async fn api_friend_status(req: &str) -> (Status, Json<Response>) {
    match fetch(req).await {
        Ok(status) => (Status::Ok, Json(Response::Success(status))),

        Err(error) => (
            Status::InternalServerError,
            Json(Response::Error(error.to_string())),
        ),
    }
}

async fn fetch(req: &str) -> Result<ResStatus> {
    split_colon!(req, [auth, user]);

    let (_, token) = find_matched_data(auth)?;

    let res = request(
        reqwest::Method::GET,
        &format!("https://api.vrchat.cloud/api/1/user/{user}/friendStatus"),
        &token,
    )
    .await?;

    if res.status().is_success() {
        Ok(res.json().await?)
    } else {
        bail!("{}", res.text().await?)
    }
}
