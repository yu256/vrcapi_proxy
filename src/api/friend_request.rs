use super::utils::request;
use crate::{split_colon, validate};
use anyhow::Result;

pub(crate) async fn api_friend_request(req: String) -> Result<bool> {
    split_colon!(req, [auth, user, method]);
    validate!(auth, token);

    request(
        method,
        &format!("https://api.vrchat.cloud/api/1/user/{user}/friendRequest"),
        token,
    )
    .map(|_| true)
}
