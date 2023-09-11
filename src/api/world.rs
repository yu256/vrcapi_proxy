use super::utils::{find_matched_data, request};
use crate::split_colon;
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
    fn trim(&mut self) {
        self.tags.retain_mut(|tag| {
            if tag.starts_with("author_tag") {
                tag.replace_range(..11, "");
                true
            } else {
                false
            }
        });
    }
}

#[post("/world", data = "<req>")]
pub(crate) fn api_world(req: &str) -> anyhow::Result<World> {
    split_colon!(req, [auth, world]);

    let token = find_matched_data(auth)?.1;

    match request(
        "GET",
        &format!("https://api.vrchat.cloud/api/1/worlds/{world}"),
        &token,
    )?
    .into_json::<World>()
    {
        Ok(mut world) => Ok({
            world.trim();
            world
        }),
        Err(err) => Err(err.into()),
    }
}
