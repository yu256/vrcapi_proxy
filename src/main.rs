#![feature(lazy_cell)]

use anyhow::Result;
use api::{fetch_friends, route, FRIENDS};
use cors::CorsConfig;
use general::{read_json, write_json, DATA_PATH};
use rocket::tokio;
use std::{collections::HashMap, sync::Arc};
use websocket::stream::stream;

mod api;
mod consts;
mod cors;
mod general;
mod macros;
mod websocket;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    init().unwrap();
    read_json::<HashMap<String, String>>("data.json")
        .unwrap()
        .into_iter()
        .for_each(spawn);
    rocket::build().mount("/", route()).attach(cors::Cors)
}

fn init() -> Result<()> {
    if DATA_PATH.join("data.json").is_file() && DATA_PATH.join("config.json").is_file() {
        return Ok(());
    }

    let conf = CorsConfig {
        url: "http://localhost:3000".to_owned(),
    };
    let data: HashMap<String, String> = HashMap::new();

    write_json(&conf, "config")?;
    write_json(&data, "data")?;

    std::process::exit(0);
}

pub(crate) fn spawn(data: (String, String)) {
    tokio::spawn(async move {
        let data = Arc::new(data);
        if let Ok(mut friends) = fetch_friends(&data.1).await {
            friends.retain(|friend| friend.location != "offline" && friend.status != "ask me");
            FRIENDS.write().await.insert(data.0.clone(), friends);
            loop {
                if let Err(e) = stream(Arc::clone(&data)).await {
                    if e.to_string()
                        .contains("invalid Auth")
                    {
                        FRIENDS.write().await.remove(&data.0);
                        break;
                    }
                }
            }
        }
    });
}
