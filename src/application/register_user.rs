use crate::{domain::user::User, dto::user::UserPayload};

pub fn register(payload: UserPayload) -> Result<String, String> {
    Ok("good".into())
}