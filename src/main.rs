#![feature(lazy_cell)]

use anyhow::Result;
use cors::CorsConfig;
use data::DATA_PATH;
use general::write_json;

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
    rocket::build()
        .mount(
            "/",
            routes![
                api::api_auth,
                api::api_twofactor_email,
                api::api_friends,
                api::api_user,
                api::api_instance,
                api::api_toggle,
                api::api_check_askme,
                api::api_search_user,
                api::api_friend_request,
                api::api_del_friend_request,
                api::api_friend_status,
                api::api_notifications,
                api::api_friend_accept,
                api::api_world,
                api::api_group,
                api::api_add_favorites
            ],
        )
        .attach(cors::CORS)
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
