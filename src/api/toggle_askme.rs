use super::utils::{find_matched_data, update_data_property, StrExt as _};
use anyhow::Result;
use rocket::http::Status;

#[post("/askme", data = "<req>")]
pub(crate) async fn api_toggle(req: &str) -> (Status, String) {
    match toggle(req) {
        Ok(bool) => (Status::Ok, format!("{}に変更しました。", bool)),

        Err(err) => (Status::InternalServerError, err.to_string()),
    }
}

fn toggle(req: &str) -> Result<bool> {
    let (auth, req) = req.split_colon()?;

    let bool = req == "true";

    update_data_property(auth, |data| {
        data.askme = bool;
    })?;

    Ok(bool)
}

#[post("/check_askme", data = "<req>")]
pub(crate) async fn api_check_askme(req: &str) -> (Status, String) {
    match check(req) {
        Ok(bool) => (Status::Ok, bool.to_string()),

        Err(err) => (Status::InternalServerError, err.to_string()),
    }
}

fn check(req: &str) -> Result<bool> {
    Ok(find_matched_data(req)?.askme)
}
