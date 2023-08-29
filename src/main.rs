#![feature(lazy_cell)]

use anyhow::Result;
use api::{fetch_friends, route, FRIENDS};
use cors::CorsConfig;
use general::{read_json, write_json, DATA_PATH};
use rocket::tokio;
use std::{collections::HashMap, sync::Arc};
use stream::stream;

mod api;
mod consts;
mod cors;
mod general;
mod macros;
mod stream;

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
    if DATA_PATH.is_dir() {
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
        if let Ok(friends) = fetch_friends(&data.1).await {
            FRIENDS.write().await.insert((*data).0.clone(), friends);
            loop {
                if let Err(e) = stream(data.clone()).await {
                    let e = e.to_string();
					println!("Error: {e}"); // debug
                    if e.contains("Missing Credentials") {
                        break;
                    } else if !e.contains("Connection reset without closing handshake") {
                        panic!("Unknown Error found: {e}");
                    }
                }
            }
        }
    });
}
