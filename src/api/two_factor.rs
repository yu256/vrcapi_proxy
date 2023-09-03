use super::utils::CLIENT;
use crate::{
    api::response::ApiResponse,
    consts::{COOKIE, UA, UA_VALUE},
    general::{read_json, HashMapExt as _},
    into_err, spawn, split_colon,
};
use anyhow::{bail, ensure, Result};
use rocket::{http::Status, serde::json::Json};
use serde_json::json;
use std::collections::HashMap;

#[post("/twofactor", data = "<req>")]
pub(crate) async fn api_twofactor(req: &str) -> (Status, Json<ApiResponse<String>>) {
    match fetch(req).await {
        Ok((auth, token)) => {
            if let Err(err) = update(auth, token) {
                return (Status::InternalServerError, Json(into_err!(err)));
            }

            (Status::Ok, Json(auth.into()))
        }

        Err(err) => (Status::InternalServerError, Json(into_err!(err))),
    }
}

async fn fetch(req: &str) -> Result<(&str, &str)> {
    split_colon!(req, [token, r#type, f, auth]);

    ensure!(auth.len() <= 50, "認証IDが長すぎます。");

    let res = CLIENT
        .post(&format!(
            "https://api.vrchat.cloud/api/1/auth/twofactorauth/{type}/verify"
        ))
        .set(UA, UA_VALUE)
        .set(COOKIE, token)
        .send_json(json!({ "code": f }))?;

    if res.status() == 200 {
        Ok((auth, token))
    } else {
        bail!("{}", res.into_string()?)
    }
}

fn update(auth: &str, token: &str) -> Result<()> {
    let mut data: HashMap<String, String> = read_json("data.json")?;

    data.add(auth, token)?;

    spawn(unsafe { data.remove_entry(auth).unwrap_unchecked() });

    Ok(())
}
