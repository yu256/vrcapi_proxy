use crate::{
    consts::{COOKIE, UA, UA_VALUE},
    general::read_json,
};
use anyhow::{Context as _, Result};
use std::{collections::HashMap, sync::LazyLock};
use ureq::Response;

pub(crate) static CLIENT: LazyLock<ureq::Agent> =
    LazyLock::new(|| ureq::AgentBuilder::new().build());

pub(crate) fn request(method: &str, target: &str, cookie: &str) -> Result<Response, ureq::Error> {
    CLIENT
        .request(method, target)
        .set(UA, UA_VALUE)
        .set(COOKIE, cookie)
        .call()
}

pub(crate) fn find_matched_data(auth: &str) -> Result<(String, String)> {
    let mut data: HashMap<String, String> = read_json("data.json")?;

    let matched = data.remove_entry(auth).with_context(|| format!("{auth}での認証に失敗しました。サーバー側の初回fetchに失敗しているか、トークンが無効です。"))?;

    Ok(matched)
}
