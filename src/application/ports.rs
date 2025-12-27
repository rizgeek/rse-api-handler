pub trait PasswordHasher {
    fn hash(&self, plain: &str) -> String;
}

pub trait Cache {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&self, key: &str, value: &str);
    fn delete(&self, key: &str);
}