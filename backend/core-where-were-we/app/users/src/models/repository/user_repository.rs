use crate::models::user::User;
use crate::models::user_id::UserId;
use anyhow::Result;

pub trait UserRepository: Send + Sync + 'static {
    async fn find_by_id(&self, user_id: &UserId) -> Result<Option<User>>;
    async fn save(&self, user: &User) -> Result<()>;
}