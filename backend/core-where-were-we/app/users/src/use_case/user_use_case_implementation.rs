//! create a new user
use crate::dtos::user_dto::UserDto;
use crate::errors::errors::UsersError;
use crate::use_case::user_use_cases::UserUseCases;
use crate::models::repository::user_repository::UserRepository;
use crate::models::user::{User, Username};
use crate::models::user_id::UserId;

pub struct CreateUserUseCaseInteractor<R> {
    user_repository: R
}

impl <R> CreateUserUseCaseInteractor<R> {
    pub fn new(user_repository: R) -> Self { Self { user_repository } }
}

impl <R> UserUseCases for CreateUserUseCaseInteractor<R>
    where R: UserRepository,
{
    async fn create(&self, name: &str, email: &str) -> Result<UserDto, UsersError> {
        let user_id = UserId::generate();
        let username = match Username::try_from(name) {
            Ok(n) => n,
            Err(e) => return Err(e)
        };
        
        let user = User::new(&user_id, &username, email, None);
        
        let _ = self.user_repository.save(&user).await;
       
        Ok(UserDto::from(user))
    } 
    
    /// add a new partner
    async fn add_partner(&self, user_id: &UserId, partner_id: &UserId) -> Result<UserDto, UsersError> {
        
        let find_result = self.user_repository.find_by_id(user_id).await?;
        
        let mut user = match find_result {
            Some(user) => user,
            None => return Err(UsersError::UserNotFind)
        };
        
        user.add_partner(partner_id);
        
        let _ = self.user_repository.save(&user);
        
        return Ok(UserDto::from(user))
    }
}