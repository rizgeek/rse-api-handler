
use std::sync::Arc;

use crate::{application::ports::PasswordHasher, infrastructure::security::bcrypt_hasher::BcryptHasher, server::app_state::AppState};

fn hasher() -> Box<dyn PasswordHasher + Send + Sync> {
    Box::new(BcryptHasher)
}

pub fn build_state() -> Arc<AppState> {
    let hasher = hasher();

    Arc::new(AppState {
        hasher,
    })
}