use super::{
    utils::{find_matched_data, request},
    FRIENDS,
};
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

#[derive(Serialize)]
pub(crate) enum Response {
    Success(ResponseInstance),
    Error(String),
}

#[post("/instance", data = "<req>")]
pub(crate) async fn api_instance(req: &str) -> (Status, Json<Response>) {
    match fetch(req).await {
        Ok((data, users)) => (Status::Ok, Json(Response::Success(data.to_res(users)))),

        Err(error) => (
            Status::InternalServerError,
            Json(Response::Error(error.to_string())),
        ),
    }
}

async fn fetch(req: &str) -> Result<(InstanceData, HashMap<String, String>)> {
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
            .context("failed to auth.")?
            .iter()
            .filter_map(|user| {
                if user.location == instance {
                    Some((user.get_img(), user.displayName.clone()))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();

        Ok((res.json().await?, users))
    } else {
        bail!("{}", res.text().await?)
    }
}
