use super::utils::{update_data_property, StrExt as _, CLIENT};
use crate::{
    consts::{COOKIE, UA, UA_VALUE},
    data::{Data, DataVecExt as _},
    general::get_data,
};
use anyhow::{bail, Error, Result};
use rocket::{http::Status, serde::json::Json};
use serde::Serialize;
use serde_json::json;
use uuid::Uuid;

const URL: &str = "https://api.vrchat.cloud/api/1/auth/twofactorauth/emailotp/verify";

#[derive(Serialize)]
pub(crate) enum Res {
    Success(String),
    Error(String),
}

impl From<Error> for Res {
    fn from(error: Error) -> Self {
        Res::Error(error.to_string())
    }
}

#[post("/twofactor_email", data = "<req>")]
pub(crate) async fn api_twofactor_email(req: &str) -> (Status, Json<Res>) {
    match req.split_once(';') {
        Some((req, auth)) => match fetch(req).await {
            Ok(token) => {
                if let Err(err) = update(token, auth) {
                    return (Status::InternalServerError, Json(Res::from(err)));
                }

                (Status::Ok, Json(Res::Success(auth.to_string())))
            }

            Err(err) => (Status::InternalServerError, Json(Res::from(err))),
        },

        None => match fetch(req).await {
            Ok(token) => {
                let auth = Uuid::new_v4().to_string();

                if let Err(err) = add(token, &auth) {
                    return (Status::InternalServerError, Json(Res::from(err)));
                }

                (Status::Ok, Json(Res::Success(auth)))
            }

            Err(err) => (Status::InternalServerError, Json(Res::from(err))),
        },
    }
}

async fn fetch(req: &str) -> Result<&str> {
    let (token, f) = req.split_colon()?;
    let res = CLIENT
        .post(URL)
        .header(UA, UA_VALUE)
        .header(COOKIE, token)
        .json(&json!({ "code": f }))
        .send()
        .await?;

    if res.status().is_success() {
        Ok(token)
    } else {
        bail!("{}", res.text().await?)
    }
}

fn update(token: &str, auth: &str) -> Result<()> {
    update_data_property(auth, |data| {
        data.token = token.to_string();
    })?;

    Ok(())
}

fn add(token: &str, auth: &str) -> Result<()> {
    let new_data = Data {
        auth: auth.to_string(),
        token: token.to_string(),
        askme: false,
    };

    let mut data: Vec<Data> = get_data::<Vec<Data>>("data")?;

    data.push(new_data);

    data.write()?;

    Ok(())
}
