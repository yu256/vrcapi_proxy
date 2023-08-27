use super::utils::{find_matched_data, request, StrExt as _};
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

#[derive(Serialize)]
pub(crate) enum Response {
    Success(World),
    Error(String),
}

#[post("/world", data = "<req>")]
pub(crate) async fn api_world(req: &str) -> (Status, Json<Response>) {
    match fetch(req).await {
        Ok(status) => (Status::Ok, Json(Response::Success(status))),

        Err(error) => (
            Status::InternalServerError,
            Json(Response::Error(error.to_string())),
        ),
    }
}

async fn fetch(req: &str) -> Result<World> {
    let (auth, world) = req.split_colon()?;

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
