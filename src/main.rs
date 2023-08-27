#![feature(lazy_cell)]

use anyhow::Result;
use api::{fetch_friends, route, FRIENDS};
use cors::CorsConfig;
use general::{read_json, write_json, DATA_PATH};
use rocket::tokio::{self, time::sleep};
use std::collections::HashMap;
// use stream::stream;

mod api;
mod consts;
mod cors;
mod general;
// mod stream;

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
        return migrate();
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
        let (auth, token) = data;
        if let Ok(friends) = fetch_friends(&token).await {
            FRIENDS.write().await.insert(auth.clone(), friends);
            loop {
                sleep(std::time::Duration::from_secs(60)).await;
                match fetch_friends(&token).await {
                    Ok(f) => {
                        let mut unlocked = FRIENDS.write().await;
                        *unlocked.get_mut(&auth).unwrap() = f;
                    }
                    Err(e) => {
                        if e.to_string().contains("Missing Credentials") {
                            break;
                        }
                    }
                }
            }
        }
    });
}

// いずれ消す
fn migrate() -> Result<()> {
    #[derive(serde::Deserialize)]
    struct OldData {
        auth: String,
        token: String,
    }
    match read_json::<Vec<OldData>>("data.json") {
        Ok(data) => {
            let mut map = HashMap::new();

            data.into_iter().for_each(|data| {
                if !data.auth.is_empty() {
                    map.insert(data.auth, data.token);
                }
            });

            write_json(&map, "data")?;

            Ok(())
        }
        Err(_) => Ok(()),
    }
}
