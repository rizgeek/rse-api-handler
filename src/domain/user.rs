pub enum UserBuildError {
    PasswordMismatch
}

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

impl User {
    pub fn new(name: String, email: String, password_hash: String) -> Self {
        Self {
            name,
            email,
            password_hash   
        }
    }
}