use super::{
    response::ApiResponse,
    utils::{find_matched_data, request},
};
use crate::{into_err, split_colon};
use anyhow::{bail, Result};
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub(crate) struct World {
    authorId: String,
    // authorName: String,
    capacity: u32,
    created_at: String,
    description: String,
    favorites: u32,
    featured: bool,
    heat: u32,
    // id: String,
    imageUrl: String,
    // instances: Option<Vec<Instance>>,
    labsPublicationDate: String,
    name: String,
    namespace: String,
    // occupants: u32,
    organization: String,
    popularity: u32,
    // previewYoutubeId: Option<String>,
    privateOccupants: u32,
    publicOccupants: u32,
    publicationDate: String,
    // releaseStatus: String,
    tags: Vec<String>,
    thumbnailImageUrl: String,
    // unityPackages: Vec<UnityPackage>,
    updated_at: String,
    // version: u32,
    visits: u32,
}

impl World {
    fn to_res(mut self) -> Self {
        self.tags.retain(|tag| tag.starts_with("author_tag"));
        self.tags.iter_mut().for_each(|tag| {
            tag.replace_range(..11, "");
        });

        self
    }
}

#[post("/world", data = "<req>")]
pub(crate) async fn api_world(req: &str) -> (Status, Json<ApiResponse<World>>) {
    match fetch(req).await {
        Ok(status) => (Status::Ok, Json(status.into())),

        Err(error) => (Status::InternalServerError, Json(into_err!(error))),
    }
}

async fn fetch(req: &str) -> Result<World> {
    split_colon!(req, [auth, world]);

    let (_, token) = find_matched_data(auth)?;

    let res = request(
        reqwest::Method::GET,
        &format!("https://api.vrchat.cloud/api/1/worlds/{world}"),
        &token,
    )
    .await?;

    if res.status().is_success() {
        Ok(res.json::<World>().await?.to_res())
    } else {
        bail!("{}", res.text().await?)
    }
}
