use super::utils::request;
use crate::unsanitizer::Unsanitizer;
use crate::{split_colon, validate};
use anyhow::Result;
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
    #[serde(skip_serializing_if = "Option::is_none")]
    instances: Option<Vec<Option<Vec<serde_json::Value>>>>,
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
    fn modify(&mut self) {
        self.tags.retain_mut(|tag| {
            if tag.starts_with("author_tag") {
                tag.replace_range(..11, "");
                true
            } else {
                false
            }
        });
        self.description = self.description.unsanitize();
    }
}

pub(crate) async fn api_world(req: String) -> Result<World> {
    split_colon!(req, [auth, world]);
    validate!(auth, token);

    match request(
        "GET",
        &format!("https://api.vrchat.cloud/api/1/worlds/{world}"),
        token,
    )?
    .into_json::<World>()
    {
        Ok(mut world) => Ok({
            world.modify();
            world
        }),
        Err(err) => Err(err.into()),
    }
}
