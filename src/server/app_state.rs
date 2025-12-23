use crate::application::ports::PasswordHasher;

pub struct AppState {
    pub hasher: Box<dyn PasswordHasher>
}