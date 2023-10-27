use super::utils::{find_matched_data, request};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub(crate) struct Notification {
    id: String,
    senderUserId: String,
    senderUsername: String,
    r#type: String,
    message: String,
    details: Option<Details>,
    seen: bool,
    created_at: String,
}

#[allow(clippy::enum_variant_names)]
#[derive(Serialize, Deserialize)]
enum Details {
    NotificationDetailInvite,
    NotificationDetailInviteResponse,
    NotificationDetailRequestInvite,
    NotificationDetailRequestInviteResponse,
    NotificationDetailVoteToKick,
}

const URL: &str = "https://api.vrchat.cloud/api/1/auth/user/notifications";

pub(crate) async fn api_notifications(req: String) -> Result<Vec<Notification>> {
    request("GET", URL, &find_matched_data(&req)?.1)?
        .into_json()
        .map_err(From::from)
}
