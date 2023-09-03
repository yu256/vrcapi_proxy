use super::utils::CLIENT;
use crate::{
    api::response::ApiResponse,
    consts::{UA, UA_VALUE},
    into_err,
};
use anyhow::{bail, Context as _, Result};
use base64::{engine::general_purpose, Engine as _};
use rocket::{http::Status, serde::json::Json};
use serde_json::Value;

const URL: &str = "https://api.vrchat.cloud/api/1/auth/user";

#[post("/auth", data = "<req>")]
pub(crate) async fn api_auth(req: &str) -> (Status, Json<ApiResponse<String>>) {
    match auth(req).await {
        Ok(token) => (Status::Ok, Json(token.into())),

        Err(error) => (Status::InternalServerError, Json(into_err!(error))),
    }
}

async fn auth(req: &str) -> Result<String> {
    let res = CLIENT
        .get(URL)
        .set(
            "Authorization",
            &format!("Basic {}", general_purpose::STANDARD_NO_PAD.encode(req)),
        )
        .set(UA, UA_VALUE)
        .call()?;

    if res.status() == 200 {
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
    } else {
        bail!("{}", res.into_string()?)
    }
}
