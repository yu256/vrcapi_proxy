use super::utils::{find_matched_data, request};
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
pub(crate) fn api_notifications(req: &str) -> anyhow::Result<Vec<Notification>> {
    request("GET", URL, &find_matched_data(req)?.1)
        .map(|res| res.into_json().map_err(From::from))?
}
