use validator::{Validate};
use serde::{Deserialize, Deserializer ,Serialize};

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserPayload {
    #[validate(length(min=1))]
    #[serde(deserialize_with = "trim_string")]
    pub name: String,

    #[validate(email)]
    #[serde(deserialize_with = "trim_string")]
    pub email: String,

    #[validate(length(min=8))]
    pub password: String,

    #[validate(length(min=8))]
    #[serde(rename="retypePassword")]
    pub retype_password: String
}


fn trim_string<'de, D>(deserializer: D) -> Result<String, D::Error> 
where 
    D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    Ok(s.trim().to_string())
}