use crate::{
    api::utils::request_json,
    general::{read_json, HashMapExt as _},
    spawn, split_colon,
};
use anyhow::ensure;
use serde_json::json;
use std::collections::HashMap;

#[post("/twofactor", data = "<req>")]
pub(crate) fn api_twofactor(req: &str) -> anyhow::Result<String> {
    split_colon!(req, [token, r#type, f, auth]);

    ensure!(auth.len() <= 50, "認証IDが長すぎます。");

    request_json(
        "POST",
        &format!("https://api.vrchat.cloud/api/1/auth/twofactorauth/{type}/verify"),
        token,
        json!({ "code": f }),
    )?;

    let mut data: HashMap<String, String> = read_json("data.json")?;

    data.add(auth, token)?;

    spawn(unsafe { data.remove_entry(auth).unwrap_unchecked() });

    Ok(auth.to_owned())
}
