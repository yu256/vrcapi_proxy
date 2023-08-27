use crate::general::read_json;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method, Status};
use rocket::{Request, Response};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

static ALLOWED_ORIGINS: LazyLock<String> =
    LazyLock::new(|| read_json::<CorsConfig>("config.json").unwrap().url);

pub(crate) struct Cors;

#[derive(Serialize, Deserialize)]
pub(crate) struct CorsConfig {
    pub(crate) url: String,
}

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if request.method() == Method::Options {
            response.set_status(Status::NoContent);
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, PATCH, GET, DELETE",
            ));
            response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        }

        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            &*ALLOWED_ORIGINS,
        ));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
