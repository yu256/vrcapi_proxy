#![feature(lazy_cell)]

use anyhow::Result;
use api::route;
use cors::CorsConfig;
use general::write_json;
use general::DATA_PATH;

mod api;
mod consts;
mod cors;
mod data;
mod general;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    init().unwrap();
    rocket::build().mount("/", route()).attach(cors::CORS)
}

fn init() -> Result<()> {
    if DATA_PATH.is_dir() {
        return Ok(());
    }

    let conf = CorsConfig {
        url: "http://localhost:3000".to_owned(),
    };
    let data: &[bool; 0] = &[]; // どうせ空なので型は適当

    write_json(&conf, "config")?;
    write_json(&data, "data")?;

    std::process::exit(0);
}
