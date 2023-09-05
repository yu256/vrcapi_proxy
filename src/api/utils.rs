use crate::{
    consts::{COOKIE, INVALID_AUTH, UA, UA_VALUE},
    general::read_json,
};
use anyhow::{Context as _, Result};
use std::{collections::HashMap, sync::LazyLock};
use ureq::Response;

pub(crate) static CLIENT: LazyLock<ureq::Agent> =
    LazyLock::new(|| ureq::AgentBuilder::new().build());

pub(crate) fn request(
    method: &str,
    target: &str,
    cookie: &str,
) -> Result<Response, ureq::Error> {
    CLIENT
        .request(method, target)
        .set(UA, UA_VALUE)
        .set(COOKIE, cookie)
        .call()
}

pub(crate) fn request_json(
    method: &str,
    target: &str,
    cookie: &str,
    data: impl serde::Serialize,
) -> Result<Response, ureq::Error> {
    CLIENT
        .request(method, target)
        .set(UA, UA_VALUE)
        .set(COOKIE, cookie)
        .send_json(data)
}

pub(crate) fn find_matched_data(auth: &str) -> Result<(String, String)> {
    let mut data: HashMap<String, String> = read_json("data.json")?;

    let matched = data.remove_entry(auth).context(INVALID_AUTH)?;

    Ok(matched)
}
