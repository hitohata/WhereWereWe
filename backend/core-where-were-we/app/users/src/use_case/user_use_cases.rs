use crate::dtos::user_dto::UserDto;
use crate::errors::errors::UsersError;
use crate::models::user_id::UserId;

pub trait UserUseCases: Send + Sync + 'static {
    async fn create(&self, name: &str, email: &str) -> Result<UserDto, UsersError>;
    async fn add_partner(&self, user_id: &str, partner_id: &str) -> Result<UserDto, UsersError>;
    async fn remove_partner(&self, user_id: &str, partner_id: &str) -> Result<UserDto, UsersError>;
}