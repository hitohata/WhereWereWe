use serde::{Deserialize, Serialize};
use crate::models::user::User;

/// User DTO
#[derive(Serialize, Deserialize)]
struct UserDto {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id.id,
            name: user.name.name,
            email: user.email,
        }
    }
}