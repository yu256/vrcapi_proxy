use crate::validate;

use super::request;
use anyhow::{Context, Result};

pub(crate) async fn api_invite_myself(req: String) -> Result<bool> {
    let (auth, id) = req
        .split_once(':')
        .context(crate::global::INVALID_REQUEST)?;
    validate!(auth, token);

    request(
        "POST",
        &format!("https://api.vrchat.cloud/api/1/invite/myself/to/{id}"),
        token,
    )
    .map(|_| true)
}
