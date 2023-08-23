use super::utils::{find_matched_data, request, StrExt as _};
use anyhow::{bail, Result};
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};

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
}

impl InstanceData {
    fn to_res(self) -> ResponseInstance {
        ResponseInstance {
            ownerId: self.ownerId,
            userCount: self.userCount,
            name: self.world.name,
            description: self.world.description,
            thumbnailImageUrl: self.world.thumbnailImageUrl,
        }
    }
}

#[derive(Serialize)]
pub(crate) enum Response {
    Success(ResponseInstance),
    Error(String),
}

#[post("/instance", data = "<req>")]
pub(crate) async fn api_instance(req: &str) -> (Status, Json<Response>) {
    match fetch(req).await {
        Ok(data) => (Status::Ok, Json(Response::Success(data.to_res()))),

        Err(error) => (
            Status::InternalServerError,
            Json(Response::Error(error.to_string())),
        ),
    }
}

async fn fetch(req: &str) -> Result<InstanceData> {
    let (auth, instance) = req.split_colon()?;

    let matched = find_matched_data(auth)?;

    let res = request(
        reqwest::Method::GET,
        &format!("https://api.vrchat.cloud/api/1/instances/{instance}"),
        &matched.token,
    )
    .await?;

    if res.status().is_success() {
        Ok(res.json().await?)
    } else {
        bail!("{}", res.text().await?)
    }
}
