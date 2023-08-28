use super::{
    utils::{find_matched_data, request},
    FRIENDS,
};
use crate::{consts::VRC_P, split_colon};
use anyhow::{bail, Context as _, Result};
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.vrchat.cloud/api/1/users/";

#[allow(non_snake_case)]
#[derive(Deserialize, Clone)]
pub(crate) struct User {
    pub(crate) bio: String,
    pub(crate) bioLinks: Vec<String>,
    pub(crate) currentAvatarThumbnailImageUrl: String,
    pub(crate) displayName: String,
    pub(crate) id: String,
    pub(crate) isFriend: bool,
    pub(crate) location: String,
    pub(crate) status: String,
    pub(crate) statusDescription: String,
    pub(crate) tags: Vec<String>,
    pub(crate) userIcon: String,
    pub(crate) profilePicOverride: String,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(crate) struct ResUser {
    bio: String,
    bioLinks: Vec<String>,
    currentAvatarThumbnailImageUrl: String,
    displayName: String,
    isFriend: bool,
    location: String,
    status: String,
    statusDescription: String,
    rank: String,
}

#[derive(Serialize)]
pub(crate) enum Response {
    Success(ResUser),
    Error(String),
}

#[post("/user", data = "<req>")]
pub(crate) async fn api_user(req: &str) -> (Status, Json<Response>) {
    match fetch(req).await {
        Ok(user) => (Status::Ok, Json(Response::Success(user))),

        Err(error) => (
            Status::InternalServerError,
            Json(Response::Error(error.to_string())),
        ),
    }
}

async fn fetch(req: &str) -> Result<ResUser> {
    split_colon!(req, [auth, user]);

    if let Some(user) = FRIENDS
        .read()
        .await
        .get(auth)
        .context("failed to auth.")?
        .iter()
        .find(|u| u.id == user)
    {
        return Ok(user.clone().to_user());
    }

    let (_, token) = unsafe { find_matched_data(auth).unwrap_unchecked() };

    let res = request(reqwest::Method::GET, &format!("{}{}", URL, user), &token).await?;

    if res.status().is_success() {
        Ok(res.json::<User>().await?.to_user())
    } else {
        bail!("{}", res.text().await?)
    }
}

impl User {
    fn to_user(self) -> ResUser {
        let mut rank = {
            let mut rank = None;
            for tag in self.tags.iter().rev() {
                match tag.as_str() {
                    "system_trust_veteran" => {
                        rank = Some("Trusted");
                        break;
                    }
                    "system_trust_trusted" => {
                        rank = Some("Known");
                        break;
                    }
                    "system_trust_known" => {
                        rank = Some("User");
                        break;
                    }
                    "system_trust_basic" => {
                        rank = Some("New User");
                        break;
                    }
                    "system_troll" => {
                        rank = Some("Troll");
                        break;
                    }
                    _ => {}
                }
            }

            rank.unwrap_or("Visitor").to_owned()
        };

        if self.tags.iter().any(|tag| tag == VRC_P) {
            rank += " VRC+"
        }

        ResUser {
            currentAvatarThumbnailImageUrl: self.get_img(),
            bio: self.bio,
            bioLinks: self.bioLinks,
            displayName: self.displayName,
            isFriend: self.isFriend,
            location: self.location,
            status: self.status,
            statusDescription: self.statusDescription,
            rank,
        }
    }
}
