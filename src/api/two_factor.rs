use crate::{
    api::utils::request_json,
    general::{read_json, HashMapExt as _},
    spawn, split_colon,
};
use anyhow::{ensure, Result};
use serde_json::json;
use std::collections::HashMap;

#[post("/twofactor", data = "<req>")]
pub(crate) fn api_twofactor(req: &str) -> anyhow::Result<String> {
    let (auth, token) = fetch(req)?;
    update(auth, token)?;

    Ok(auth.to_owned())
}

fn fetch(req: &str) -> Result<(&str, &str)> {
    split_colon!(req, [token, r#type, f, auth]);

    ensure!(auth.len() <= 50, "認証IDが長すぎます。");

    request_json(
        "POST",
        &format!("https://api.vrchat.cloud/api/1/auth/twofactorauth/{type}/verify"),
        token,
        json!({ "code": f }),
    )?;

    Ok((auth, token))
}

fn update(auth: &str, token: &str) -> Result<()> {
    let mut data: HashMap<String, String> = read_json("data.json")?;

    data.add(auth, token)?;

    spawn(unsafe { data.remove_entry(auth).unwrap_unchecked() });

    Ok(())
}
