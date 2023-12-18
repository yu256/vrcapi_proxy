use super::utils::request;
use crate::{split_colon, validate};
use anyhow::Result;

pub(crate) async fn api_friend_accept(req: String) -> Result<()> {
    split_colon!(req, [auth, id]);
    validate!(auth, token);

    request(
        "PUT",
        &format!("https://api.vrchat.cloud/api/1/auth/user/notifications/{id}/accept"),
        token,
    )?;

    Ok(())
}
