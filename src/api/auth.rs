use super::utils::CLIENT;
use crate::consts::{UA, UA_VALUE};
use anyhow::{bail, Context as _, Result};
use base64::{engine::general_purpose, Engine as _};
use rocket::{http::Status, serde::json::Json};
use serde::Serialize;
use serde_json::Value;

const URL: &str = "https://api.vrchat.cloud/api/1/auth/user";

const ON_ERROR: &str = "An error occurred while parsing the cookie.";

#[derive(Serialize)]
pub(crate) enum Response {
    Success(String),
    Error(String),
}

#[post("/auth", data = "<req>")]
pub(crate) async fn api_auth(req: &str) -> (Status, Json<Response>) {
    match auth(req).await {
        Ok(token) => (Status::Ok, Json(Response::Success(token))),

        Err(error) => (
            Status::InternalServerError,
            Json(Response::Error(error.to_string())),
        ),
    }
}

async fn auth(req: &str) -> Result<String> {
    let res = CLIENT
        .get(URL)
        .header(
            "Authorization",
            format!("Basic {}", general_purpose::STANDARD_NO_PAD.encode(req)),
        )
        .header(UA, UA_VALUE)
        .send()
        .await?;

    if res.status().is_success() {
        let token = String::from("auth=")
            + res
                .headers()
                .get("set-cookie")
                .context(ON_ERROR)?
                .to_str()?
                .split(';')
                .next()
                .context(ON_ERROR)?
                .split('=')
                .nth(1)
                .context(ON_ERROR)?;

        let auth_type = {
            let json: Value = res.json().await?;
            json["requiresTwoFactorAuth"]
                .as_array()
                .and_then(|arr| arr.get(0))
                .context("No 2FA")?
                .as_str()
                .context("No 2FA")?
                .to_lowercase()
        };

        Ok(token + ":" + &auth_type)
    } else {
        bail!("{}", res.text().await?)
    }
}
