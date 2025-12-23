use axum::{Json, http::StatusCode};
use validator::Validate;

use crate::{ application::register_user::register, dto::user::UserPayload, helper::api_response::{ApiResponse, ApiResponseWithStatus}};

pub async fn hdl_register(Json(payload): Json<UserPayload>) -> ApiResponseWithStatus {	

	// check payload
	match payload.validate() {
		Ok(_) => {
			match register(payload) {
				Ok(message) => ApiResponseWithStatus { 
					status: StatusCode::CREATED, 
					response: ApiResponse::Success { message } 
				},
				Err(error) => ApiResponseWithStatus { 
					status: StatusCode::BAD_REQUEST,
					response: ApiResponse::Error { error } 
				}
			}
		},
		Err(e) => {
			return ApiResponseWithStatus {
                status: StatusCode::BAD_REQUEST,
                response: ApiResponse::Error {
                    error: e.to_string(),
                },
            };
		}
	}
}
