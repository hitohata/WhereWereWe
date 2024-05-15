use serde::{Deserialize, Serialize};
use crate::models::user::User;

/// User DTO
#[derive(Serialize, Deserialize)]
pub struct UserDto {
    pub id: String,
    pub name: String,
    pub email: String,
    pub partners: Vec<String>
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id().id().to_owned(),
            name: user.name().to_string(),
            email: user.email().to_string(),
            partners: user.partners().into_iter().map(|id| id.id().to_owned()).collect::<Vec<String>>()
        }
    }
}