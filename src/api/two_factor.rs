use crate::{
    api::utils::request_json,
    general::{read_json, HashMapExt as _},
    spawn, split_colon,
};
use anyhow::{ensure, Result};
use serde_json::json;
use std::collections::HashMap;

pub(crate) async fn api_twofactor(req: String) -> Result<String> {
    split_colon!(req, [token, r#type, f, auth]);

    ensure!(auth.chars().count() <= 50, "認証IDが長すぎます。");

    request_json(
        "POST",
        &format!("https://api.vrchat.cloud/api/1/auth/twofactorauth/{type}/verify"),
        token,
        json!({ "code": f }),
    )?;

    let mut data: HashMap<String, String> = read_json("data.json")?;

    data.add(auth, token)?;

    // Safety: 前の行で追加したものなので必ずSomeである
    spawn(unsafe { data.remove_entry(auth).unwrap_unchecked() });

    Ok(auth.to_owned())
}
