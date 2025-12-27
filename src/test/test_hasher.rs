#[cfg(test)]
mod tests {
    use crate::application::ports::PasswordHasher;
    use crate::infrastructure::security::bcrypt_hasher::BcryptHasher;

    #[test]
    fn test_hash() {
        let hasher = BcryptHasher;
        let hashed = hasher.hash("secret");
        assert!(!hashed.is_empty());
    }
}
