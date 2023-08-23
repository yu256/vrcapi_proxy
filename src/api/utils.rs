use crate::{
    consts::{COOKIE, UA, UA_VALUE},
    data::{Data, DataVecExt as _},
    general::get_data,
};
use anyhow::{bail, Context as _, Result};
use reqwest::Response;
use std::sync::LazyLock;

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

pub(crate) fn find_matched_data(auth: &str) -> Result<Data> {
    let data: Vec<Data> = get_data("data")?;

    let matched: Data = data
        .into_iter()
        .find(|data| data.auth == auth)
        .context(NO_AUTH)?;

    Ok(matched)
}

pub(crate) fn update_data_property<T>(auth: &str, updater: impl Fn(&mut Data) -> T) -> Result<()> {
    let mut data: Vec<Data> = get_data::<Vec<Data>>("data")?;

    if let Some(data) = data.iter_mut().find(|data| data.auth == auth) {
        updater(data);
    } else {
        bail!(NO_AUTH);
    }

    data.write()?;

    Ok(())
}

pub(crate) trait StrExt {
    fn split_colon(&self) -> Result<(&str, &str)>;
}

impl StrExt for str {
    fn split_colon(&self) -> Result<(&str, &str)> {
        Ok(self.split_once(':').context(INVALID_INPUT)?)
    }
}
