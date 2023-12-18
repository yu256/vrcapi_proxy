use super::utils::request;
use crate::unsanitizer::Unsanitizer;
use crate::{global::FRIENDS, validate};
use anyhow::{Context, Result};
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
            description: self.world.description.unsanitize(),
            thumbnailImageUrl: self.world.thumbnailImageUrl,
            users,
        }
    }
}

pub(crate) async fn api_instance(req: String) -> Result<ResponseInstance> {
    let (auth, instance) = req
        .split_once(':')
        .context(crate::global::INVALID_REQUEST)?;
    validate!(auth, token);

    let res = request(
        "GET",
        &format!("https://api.vrchat.cloud/api/1/instances/{instance}"),
        token,
    )?;

    let users = FRIENDS
        .read(|friends| {
            friends
                .iter()
                .filter(|user| user.location == instance)
                .map(|user| {
                    (
                        if !user.userIcon.is_empty() {
                            user.userIcon.clone()
                        } else if !user.profilePicOverride.is_empty() {
                            user.profilePicOverride.clone()
                        } else {
                            user.currentAvatarThumbnailImageUrl.clone()
                        },
                        user.displayName.clone(),
                    )
                })
                .collect()
        })
        .await;

    Ok(res.into_json::<InstanceData>()?.into_res(users))
}
