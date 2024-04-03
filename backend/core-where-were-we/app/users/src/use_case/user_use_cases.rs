use crate::dtos::user_dto::UserDto;
use crate::errors::errors::UsersError;

pub trait UserUseCases: Send + Sync + 'static {
    async fn create(&self, name: &str, email: &str) -> Result<UserDto, UsersError>;
}