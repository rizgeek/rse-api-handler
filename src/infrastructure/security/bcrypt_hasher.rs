use bcrypt::{DEFAULT_COST, hash};

use crate::application::ports::PasswordHasher;

pub struct BcryptHasher;

impl PasswordHasher for BcryptHasher {
    fn hash(&self, plain: &str) -> String {
        hash(plain, DEFAULT_COST)
            .expect("bcrypt hashing failed")
    }
}