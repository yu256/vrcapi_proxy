use super::utils::{find_matched_data, request};
use crate::split_colon;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub(crate) struct ResStatus {
    // isFriend: bool,
    outgoingRequest: bool,
    incomingRequest: bool,
}

pub(crate) async fn api_friend_status(req: String) -> Result<ResStatus> {
    split_colon!(req, [auth, user]);

    let token = find_matched_data(auth)?.1;

    request(
        "GET",
        &format!("https://api.vrchat.cloud/api/1/user/{user}/friendStatus"),
        &token,
    )?
    .into_json()
    .map_err(From::from)
}
