use serde::{Deserialize, Serialize};
use crate::models::user::User;

/// User DTO
#[derive(Serialize, Deserialize)]
struct UserDto {
    pub id: String,
    pub name: String,
    pub email: String,
}

/// convert the user struct into the plane object
pub (crate) fn to_dto(user: User) -> UserDto {
    UserDto {
        id: user.id.id,
        name: user.name.name,
        email: user.email,
    }
}