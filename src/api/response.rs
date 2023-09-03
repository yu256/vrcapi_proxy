use serde::Serialize;

#[derive(Serialize)]
pub(crate) enum ApiResponse<T> {
    Success(T),
    Error(String),
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
