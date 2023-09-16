use crate::{
    consts::{COOKIE, INVALID_AUTH, UA, UA_VALUE},
    general::read_json,
};
use anyhow::{anyhow, Context as _, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{collections::HashMap, sync::LazyLock};
use ureq::Response;

#[derive(Deserialize)]
struct ErrorMessage {
    error: ErrorDetail,
}

#[derive(Deserialize)]
struct ErrorDetail {
    message: String,
    // status_code: u32,
}

pub(crate) static CLIENT: LazyLock<Result<ureq::Agent>> = LazyLock::new(|| {
    Ok(ureq::builder()
        .tls_connector(Arc::new(native_tls::TlsConnector::new()?))
        .build())
});

pub(crate) fn make_request(
    method: &str,
    target: &str,
    cookie: &str,
    data: Option<impl Serialize>,
) -> Result<Response> {
    let builder = CLIENT
        .as_ref()
        .map_err(|e| anyhow!("{}", e))?
        .request(method, target)
        .set(UA, UA_VALUE)
        .set(COOKIE, cookie);

    let res = if let Some(data) = data {
        builder.send_json(data)
    } else {
        builder.call()
    };

    match res {
        Ok(ok) => Ok(ok),
        Err(ureq::Error::Status(_, res)) => Err(anyhow!(
            "{}",
            res.into_json::<ErrorMessage>()?.error.message
        )),
        Err(e) => Err(e.into()),
    }
}

pub(crate) fn request(method: &str, target: &str, cookie: &str) -> Result<Response> {
    make_request(method, target, cookie, None::<&str>)
}

pub(crate) fn request_json(
    method: &str,
    target: &str,
    cookie: &str,
    data: impl Serialize,
) -> Result<Response> {
    make_request(method, target, cookie, Some(data))
}

pub(crate) fn find_matched_data(auth: &str) -> Result<(String, String)> {
    let mut data: HashMap<String, String> = read_json("data.json")?;

    let matched = data.remove_entry(auth).context(INVALID_AUTH)?;

    Ok(matched)
}
