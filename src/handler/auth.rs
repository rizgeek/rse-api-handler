use axum::{Json, http::StatusCode};
use validator::Validate;

use crate::{dto::user::User, helper::api_response::{ApiResponse, ApiResponseWithStatus}};

pub async fn hdl_register(Json(payload): Json<User>) -> ApiResponseWithStatus {	

	match payload.validate() {
		Ok(_) => (),
		Err(e) => {
			return ApiResponseWithStatus {
                status: StatusCode::BAD_REQUEST,
                response: ApiResponse::Error {
                    error: e.to_string(),
                },
            };
		}
	}

	// compare password
	if !&payload.compare_password() {
		return ApiResponseWithStatus {
			status: StatusCode::BAD_REQUEST,
			response: ApiResponse::Error {
				error: "main password and retype password not match".into()
			},
		};		
	}

	// cleanup payload
	let data = payload.cleanup_payload();

	println!("{:?}",data);
			
	return ApiResponseWithStatus {
		status: StatusCode::CREATED,
		response: ApiResponse::Success {
			message: "good".into()
		},
	};
}
