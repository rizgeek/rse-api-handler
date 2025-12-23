pub trait PasswordHasher {
    fn hash(&self, plain: &str) -> String;
}