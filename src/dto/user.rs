use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};


fn not_blank(value: &str) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        return Err(ValidationError::new("blank"));
    }
    Ok(())
}


#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct User {

    #[validate(custom(function="not_blank"))]
    #[validate(length(min=1))]
    pub name: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min=8))]
    pub password: String,

    #[validate(length(min=8))]
    #[serde(rename="retypePassword")]
    pub retype_password: String,
}

impl User {
    pub fn cleanup_payload(mut self) -> Self {
        self.name = self.name.trim().to_string();
        self.email = self.email.trim().to_string();

        self
    }

    pub fn compare_password(&self) -> bool {
        self.password == self.retype_password
    }

}