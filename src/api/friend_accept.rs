use super::utils::{find_matched_data, request};
use crate::split_colon;
use anyhow::Result;

pub(crate) async fn api_friend_accept(req: String) -> Result<()> {
    split_colon!(req, [auth, id]);

    let token = find_matched_data(auth)?.1;

    request(
        "PUT",
        &format!("https://api.vrchat.cloud/api/1/auth/user/notifications/{id}/accept"),
        &token,
    )?;

    Ok(())
}
