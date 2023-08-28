use super::utils::CLIENT;
use crate::consts::{UA, UA_VALUE};
use anyhow::{bail, Context as _, Result};
use base64::{engine::general_purpose, Engine as _};
use rocket::{http::Status, serde::json::Json};
use serde::Serialize;

const URL: &str = "https://api.vrchat.cloud/api/1/auth/user";

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
                .and_then(|c| c.to_str().ok())
                .and_then(|c| c.split(':').next())
                .and_then(|c| c.split('=').nth(1))
                .context("invalid cookie found.")?;

        Ok(token)
    } else {
        bail!("{}", res.text().await?)
    }
}
