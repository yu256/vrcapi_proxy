use super::utils::{find_matched_data, request};
use crate::{api::response::ApiResponse, into_err, split_colon};
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

#[post("/friend_status", data = "<req>")]
pub(crate) async fn api_friend_status(req: &str) -> (Status, Json<ApiResponse<ResStatus>>) {
    match fetch(req).await {
        Ok(status) => (Status::Ok, Json(status.into())),

        Err(error) => (Status::InternalServerError, Json(into_err!(error))),
    }
}

async fn fetch(req: &str) -> Result<ResStatus> {
    split_colon!(req, [auth, user]);

    let (_, token) = find_matched_data(auth)?;

    let res = request(
        "GET",
        &format!("https://api.vrchat.cloud/api/1/user/{user}/friendStatus"),
        &token,
    )?;

    if res.status() == 200 {
        Ok(res.into_json()?)
    } else {
        bail!("{}", res.into_string()?)
    }
}
