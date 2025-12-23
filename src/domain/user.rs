use crate::{application::ports::PasswordHasher, dto::user::UserPayload};

pub enum UserBuildError {
    PasswordMismatch
}

pub struct User {
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

impl User {
    pub fn from_payload(
        payload: UserPayload, hasher: &impl PasswordHasher
    ) -> Result<Self, UserBuildError> {
        
        if payload.password != payload.retype_password {
            return Err(UserBuildError::PasswordMismatch);
        }

        // let hash = hasher.hash(&payload.password);
        let hash = "good".to_string();
        
        Ok(Self {
            name: payload.name,
            email: payload.email,
            password_hash: hash
        })

    }
}