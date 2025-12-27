
use std::{env, sync::Arc};

use crate::{application::ports::{Cache, PasswordHasher}, infrastructure::{cache::redis::RedisCache, security::bcrypt_hasher::BcryptHasher}, server::app_state::AppState};

fn hasher() -> Box<dyn PasswordHasher + Send + Sync> {
    Box::new(BcryptHasher)
}

fn cache() -> Box<dyn Cache + Send + Sync> {
    let cache_url: String = env::var("REDIS_URL").expect("REDIS_URL must be set in .env");

    Box::new(RedisCache::new(cache_url.as_str()))
}

pub fn build_state() -> Arc<AppState> {
    let hasher = hasher();
    
    let cache = cache();

    Arc::new(AppState {
        hasher,
        cache
    })
}