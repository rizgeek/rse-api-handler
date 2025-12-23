use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;


#[derive(Serialize)]
pub enum ApiResponse {
    Success { message: String },
    Error { error: String},
}

pub struct ApiResponseWithStatus {
    pub status: StatusCode,
    pub response: ApiResponse
}

impl IntoResponse for ApiResponseWithStatus  {
    fn into_response(self) -> axum::response::Response {
        let body = Json(self.response);
        (self.status, body).into_response()
    }
}