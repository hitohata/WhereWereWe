use crate::dtos::user_dto::UserDto;
use crate::errors::errors::UsersError;

pub trait UserUseCases: Send + Sync + 'static {
    async fn create(&self, name: &str, email: &str) -> Result<UserDto, UsersError>;
    async fn add_partner(&self, user_id: &str, partner_id: &str) -> Result<UserDto, UsersError>;
    async fn remove_partner(&self, user_id: &str, partner_id: &str) -> Result<UserDto, UsersError>;
    async fn change_name(&self, user_id: &str, new_name: &str) -> Result<UserDto, UsersError>;
}