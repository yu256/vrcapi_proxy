use crate::{
    consts::{COOKIE, UA, UA_VALUE},
    general::read_json,
};
use anyhow::{Context as _, Result};
use reqwest::Response;
use std::{collections::HashMap, sync::LazyLock};

const INVALID_INPUT: &str = "Invalid input format.";

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

const NO_AUTH: &str = "Failed to auth.";

pub(crate) fn find_matched_data(auth: &str) -> Result<(String, String)> {
    let mut data: HashMap<String, String> = read_json("data.json")?;

    let matched = data.remove_entry(auth).context(NO_AUTH)?;

    Ok(matched)
}

pub(crate) trait StrExt {
    fn split_colon(&self) -> Result<(&str, &str)>;
}

impl StrExt for str {
    fn split_colon(&self) -> Result<(&str, &str)> {
        self.split_once(':').context(INVALID_INPUT)
    }
}
