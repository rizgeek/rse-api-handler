use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use validator::Validate;

use crate::{ application::register_user::register, dto::user::UserPayload, helper::api_response::{ApiResponse, ApiResponseWithStatus}, server::app_state::AppState};

pub async fn hdl_register(
	State(state): State<Arc<AppState>>,
	Json(payload): Json<UserPayload>
) -> ApiResponseWithStatus {	

	// check payload
	match payload.validate() {
		Ok(_) => {
			match register(state, payload) {
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
