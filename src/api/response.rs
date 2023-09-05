use anyhow::Result;
use rocket::http::{ContentType, Status};
use rocket::response::{self, Responder, Response};
use rocket::Request;
use serde::Serialize;
use std::io::Cursor;

impl<T> From<Result<T>> for ApiResponse<T> {
    fn from(value: Result<T>) -> Self {
        match value {
            Ok(ok) => ok.into(),

            Err(error) => ApiResponse::Error(format!("{{\"Error\":\"{}\"}}", error)),
        }
    }
}

#[derive(Serialize)]
pub(crate) enum ApiResponse<T> {
    Success(T),
    Error(String),
}

impl<'a, T: serde::Serialize> Responder<'a, 'a> for ApiResponse<T> {
    fn respond_to(self, _: &Request) -> response::Result<'a> {
        match self {
            ApiResponse::Success(data) => {
                let json = format!("{{\"Success\":{}}}", unsafe {
                    serde_json::to_string(&data).unwrap_unchecked()
                });
                Response::build()
                    .header(ContentType::JSON)
                    .status(Status::Ok)
                    .sized_body(json.len(), Cursor::new(json))
                    .ok()
            }
            ApiResponse::Error(error) => Response::build()
                .header(ContentType::JSON)
                .status(Status::InternalServerError)
                .sized_body(error.len(), Cursor::new(error))
                .ok(),
        }
    }
}

impl From<&str> for ApiResponse<String> {
    fn from(success: &str) -> Self {
        ApiResponse::Success(success.to_string())
    }
}

impl<T> From<T> for ApiResponse<T> {
    fn from(success: T) -> Self {
        ApiResponse::Success(success)
    }
}
