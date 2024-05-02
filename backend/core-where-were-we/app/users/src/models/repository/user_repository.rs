use mockall::*;
use crate::errors::errors::UsersError;
use crate::models::user::User;
use crate::models::user_id::UserId;

#[automock]
pub trait UserRepository: Send + Sync + 'static {
    async fn find_by_id(&self, user_id: &UserId) -> Result<Option<User>, UsersError>;
    async fn save(&self, user: &User) -> Result<(), UsersError>;
}