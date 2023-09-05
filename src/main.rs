#![feature(lazy_cell)]
#![feature(atomic_bool_fetch_not)]

use anyhow::Result;
use api::route;
use cors::CorsConfig;
use fetch_friends::spawn;
use general::{read_json, write_json, DATA_PATH};
use std::collections::HashMap;

mod api;
mod consts;
mod cors;
mod fetch_friends;
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
