use super::utils::{make_request, Header};
use anyhow::{Context as _, Result};
use base64::{engine::general_purpose, Engine as _};
use trie_match::trie_match;

const URL: &str = "https://api.vrchat.cloud/api/1/auth/user";

#[allow(non_snake_case)]
#[derive(serde::Deserialize)]
struct TwoFactor {
    requiresTwoFactorAuth: Vec<String>,
}

pub(crate) async fn api_auth(req: String) -> Result<String> {
    let res = make_request(
        "GET",
        URL,
        Header::Auth((
            "Authorization",
            &format!("Basic {}", general_purpose::STANDARD_NO_PAD.encode(req)),
        )),
        None::<()>,
    )?;

    let token = String::from("auth=")
        + res
            .header("set-cookie")
            .and_then(|c| c.split(';').next())
            .and_then(|c| c.split('=').nth(1))
            .context("invalid cookie found.")?;

    let auth_type = res
        .into_json::<TwoFactor>()?
        .requiresTwoFactorAuth
        .into_iter()
        .find_map(|auth| {
            trie_match! {
                match auth.as_str() {
                    "emailOtp" => Some("emailotp"),
                    "totp" => Some("totp"),
                    _ => None,
                }
            }
        })
        .unwrap_or("otp");

    Ok(token + ":" + &auth_type)
}
