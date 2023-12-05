use crate::{api::utils::request_json, init::Data, spawn, split_colon};
use anyhow::{ensure, Result};
use serde_json::json;

pub(crate) async fn api_twofactor(
    req: String,
    credentials: crate::types::Credentials,
) -> Result<&'static str> {
    let mut iter = req.split(':');
    split_colon!(iter, [token, r#type, f, auth]);

    ensure!(auth.chars().count() <= 50, "認証IDが長すぎます。");

    request_json(
        "POST",
        &format!("https://api.vrchat.cloud/api/1/auth/twofactorauth/{type}/verify"),
        token,
        json!({ "code": f }),
    )?;

    let data = {
        let data = crate::general::read_json::<Data>("data.json")?;
        Data {
            listen: data.listen,
            cors: data.cors,
            auth: data.auth,
            token: token.into(),
        }
    };

    crate::general::write_json::<Data>(&data, "data.json")?;

    credentials.write().await.1 = data.token;

    spawn(credentials).await;

    Ok(credentials.read().await.0)
}
