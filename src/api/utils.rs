use crate::{
    consts::{COOKIE, UA, UA_VALUE},
    general::read_json,
};
use anyhow::{Context as _, Result};
use reqwest::Response;
use std::{collections::HashMap, sync::LazyLock};

pub(crate) static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);

pub(crate) async fn request(
    method: reqwest::Method,
    target: &str,
    cookie: &str,
) -> Result<Response, reqwest::Error> {
    CLIENT
        .request(method, target)
        .header(UA, UA_VALUE)
        .header(COOKIE, cookie)
        .send()
        .await
}

pub(crate) fn find_matched_data(auth: &str) -> Result<(String, String)> {
    let mut data: HashMap<String, String> = read_json("data.json")?;

    let matched = data.remove_entry(auth).with_context(|| format!("{auth}での認証に失敗しました。サーバー側の初回fetchに失敗しているか、トークンが無効です。"))?;

    Ok(matched)
}
