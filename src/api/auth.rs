use super::utils::CLIENT;
use crate::consts::{UA, UA_VALUE};
use anyhow::Context as _;
use base64::{engine::general_purpose, Engine as _};
use serde_json::Value;

const URL: &str = "https://api.vrchat.cloud/api/1/auth/user";

#[post("/auth", data = "<req>")]
pub(crate) fn api_auth(req: &str) -> anyhow::Result<String> {
    let res = CLIENT
        .get(URL)
        .set(
            "Authorization",
            &format!("Basic {}", general_purpose::STANDARD_NO_PAD.encode(req)),
        )
        .set(UA, UA_VALUE)
        .call()?;

    let token = String::from("auth=")
        + res
            .header("set-cookie")
            .and_then(|c| c.split(';').next())
            .and_then(|c| c.split('=').nth(1))
            .context("invalid cookie found.")?;

    let auth_type = {
        let json: Value = res.into_json()?;
        json["requiresTwoFactorAuth"]
            .as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|value| value.as_str())
            .context("No 2FA")?
            .to_lowercase()
    };

    Ok(token + ":" + &auth_type)
}
