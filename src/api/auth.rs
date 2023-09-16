use super::utils::CLIENT;
use crate::consts::{UA, UA_VALUE};
use anyhow::{Context as _, Result};
use base64::{engine::general_purpose, Engine as _};

const URL: &str = "https://api.vrchat.cloud/api/1/auth/user";

#[allow(non_snake_case)]
#[derive(serde::Deserialize)]
struct TwoFactor {
    requiresTwoFactorAuth: [String; 1],
}

#[post("/auth", data = "<req>")]
pub(crate) fn api_auth(req: &str) -> Result<String> {
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

    let auth_type = res.into_json::<TwoFactor>()?.requiresTwoFactorAuth[0].to_lowercase();

    Ok(token + ":" + &auth_type)
}
