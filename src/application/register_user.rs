use std::sync::Arc;
use crate::{domain::user::User, dto::user::UserPayload, server::app_state::AppState};

pub fn register(state: Arc<AppState>, payload: UserPayload) -> Result<String, String> {
    
    // validate password and password retype
    if payload.password != payload.retype_password {
        return Err("password and retypePassword Not Match".into());
    }
    
    // hash password
    let password_hash = std::thread::spawn({
        let password = payload.password.clone();
        let hasher: Arc<AppState> = state.clone();

        move || hasher.hasher.hash(&password)
    }).join().map_err(|_| "Failed to hash password")?;

    // convert to domain users
    let user =  User::new(payload.name, payload.email, password_hash);

    println!("{:?}",user);

    Ok("User has been created".into())

}