use super::utils::{find_matched_data, request};
use crate::{api::response::ApiResponse, into_err};
use anyhow::{bail, Result};
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub(crate) struct Notification {
    created_at: String,
    details: String, // NotificationDetailInvite, NotificationDetailInviteResponse, NotificationDetailRequestInvite, NotificationDetailRequestInviteResponse, NotificationDetailVoteToKick
    id: String,
    message: String,
    seen: bool,
    receiverUserId: String,
    senderUserId: String,
    r#type: String,
}

const URL: &str = "https://api.vrchat.cloud/api/1/auth/user/notifications";

#[post("/notifications", data = "<req>")]
pub(crate) async fn api_notifications(req: &str) -> (Status, Json<ApiResponse<Vec<Notification>>>) {
    match fetch(req).await {
        Ok(notifications) => (Status::Ok, Json(notifications.into())),

        Err(error) => (Status::InternalServerError, Json(into_err!(error))),
    }
}

async fn fetch(req: &str) -> Result<Vec<Notification>> {
    let (_, token) = find_matched_data(req)?;

    let res = request(reqwest::Method::GET, URL, &token).await?;

    if res.status().is_success() {
        Ok(res.json().await?)
    } else {
        bail!("{}", res.text().await?)
    }
}
