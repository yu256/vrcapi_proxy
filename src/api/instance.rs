use super::{
    utils::{find_matched_data, request},
    FRIENDS,
};
use crate::{api::response::ApiResponse, into_err};
use anyhow::{bail, Context as _, Result};
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct World {
    name: String,
    description: String,
    thumbnailImageUrl: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct InstanceData {
    ownerId: Option<String>,
    userCount: i32,
    world: World,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(crate) struct ResponseInstance {
    ownerId: Option<String>,
    userCount: i32,
    name: String,
    description: String,
    thumbnailImageUrl: String,
    users: HashMap<String, String>,
}

impl InstanceData {
    fn to_res(self, users: HashMap<String, String>) -> ResponseInstance {
        ResponseInstance {
            ownerId: self.ownerId,
            userCount: self.userCount,
            name: self.world.name,
            description: self.world.description,
            thumbnailImageUrl: self.world.thumbnailImageUrl,
            users,
        }
    }
}

#[post("/instance", data = "<req>")]
pub(crate) async fn api_instance(req: &str) -> (Status, Json<ApiResponse<ResponseInstance>>) {
    match fetch(req).await {
        Ok(data) => (Status::Ok, Json(data.into())),

        Err(error) => (Status::InternalServerError, Json(into_err!(error))),
    }
}

async fn fetch(req: &str) -> Result<ResponseInstance> {
    let (auth, instance) = req.split_once(':').context("Failed to split")?;

    let (_, token) = find_matched_data(auth)?;

    let res = request(
        reqwest::Method::GET,
        &format!("https://api.vrchat.cloud/api/1/instances/{instance}"),
        &token,
    )
    .await?;

    if res.status().is_success() {
        let users = FRIENDS
            .read()
            .await
            .get(auth)
            .with_context(|| format!("{auth}での認証に失敗しました。サーバー側の初回fetchに失敗しているか、トークンが無効です。"))?
            .iter()
            .filter_map(|user| {
                if user.location == instance {
                    Some((user.get_img(), user.displayName.clone()))
                } else {
                    None
                }
            })
            .collect();

        Ok(res.json::<InstanceData>().await?.to_res(users))
    } else {
        bail!("{}", res.text().await?)
    }
}
