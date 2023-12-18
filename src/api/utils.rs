use crate::global::{COOKIE, UA, APP_NAME};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::LazyLock;
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

static CLIENT: LazyLock<Result<ureq::Agent>> = LazyLock::new(|| {
    Ok(ureq::builder()
        .tls_connector(Arc::new(native_tls::TlsConnector::new()?))
        .build())
});

pub(super) enum Header<'a> {
    Cookie(&'a str),
    Auth((&'a str, &'a str)),
}

pub(super) fn make_request(
    method: &str,
    target: &str,
    header: Header,
    serializable: Option<impl Serialize>,
) -> Result<Response> {
    match CLIENT.as_ref() {
        Ok(agent) => {
            let builder = agent.request(method, target).set(UA, APP_NAME);

            let builder = match header {
                Header::Cookie(cookie) => builder.set(COOKIE, cookie),
                Header::Auth((header, value)) => builder.set(header, value),
            };

            match if let Some(serializable) = serializable {
                builder.send_json(serializable)
            } else {
                builder.call()
            } {
                Ok(ok) => Ok(ok),
                Err(ureq::Error::Status(_, res)) => Err(anyhow!(
                    "{}",
                    res.into_json::<ErrorMessage>()?
                        .error
                        .message
                        .replace('\"', "")
                )),
                Err(e) => Err(e.into()),
            }
        }
        Err(e) => Err(anyhow!("{e}")),
    }
}

#[inline]
pub(crate) fn request(method: &str, target: &str, cookie: &str) -> Result<Response> {
    make_request(method, target, Header::Cookie(cookie), None::<()>)
}

#[inline]
pub(crate) fn request_json(
    method: &str,
    target: &str,
    cookie: &str,
    serializable: impl Serialize,
) -> Result<Response> {
    make_request(method, target, Header::Cookie(cookie), Some(serializable))
}
