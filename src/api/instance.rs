use super::utils::{find_matched_data, request};
use crate::get_img;
use crate::global::{FRIENDS, INVALID_AUTH};
use anyhow::{Context as _, Result};
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
    fn into_res(self, users: HashMap<String, String>) -> ResponseInstance {
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

pub(crate) async fn api_instance(req: String) -> Result<ResponseInstance> {
    let (auth, instance) = req.split_once(':').context("Failed to split")?;

    let token = find_matched_data(auth)?.1;

    let res = request(
        "GET",
        &format!("https://api.vrchat.cloud/api/1/instances/{instance}"),
        &token,
    )?;

    let users = FRIENDS
        .read()
        .await
        .get(auth)
        .context(INVALID_AUTH)?
        .iter()
        .filter_map(|user| {
            if user.location == instance {
                Some((get_img!(user, clone), user.displayName.clone()))
            } else {
                None
            }
        })
        .collect();

    Ok(res.into_json::<InstanceData>()?.into_res(users))
}
