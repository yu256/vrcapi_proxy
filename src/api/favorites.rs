use super::utils::find_matched_data;
use crate::{
    api::{response::ApiResponse, utils::request_json},
    split_colon,
};
use anyhow::anyhow;
use serde_json::json;

#[post("/favorites", data = "<req>")]
pub(crate) fn api_add_favorites(req: &str) -> ApiResponse<bool> {
    (|| {
        split_colon!(req, [auth, r#type, id, tag]);

        let token = find_matched_data(auth)?.1;

        match request_json(
            "POST",
            "https://api.vrchat.cloud/api/1/favorites",
            &token,
            json!( {"type": r#type, "favoriteId": id, "tags": [tag]} ),
        ) {
            Ok(_) => Ok(true),
            Err(ureq::Error::Status(400, _)) => Err(anyhow!("既に登録されています。")),
            Err(e) => Err(e.into()),
        }
    })()
    .into()
}
