
use crate::application::ports::{Cache, PasswordHasher};

pub struct AppState {
    pub hasher: Box<dyn PasswordHasher + Send + Sync>,
    pub cache: Box<dyn Cache + Send + Sync>
}