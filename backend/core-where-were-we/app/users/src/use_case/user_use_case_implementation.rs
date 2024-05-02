//! create a new user
use tokio::try_join;

use crate::dtos::user_dto::UserDto;
use crate::errors::errors::UsersError;
use crate::models::repository::user_repository::UserRepository;
use crate::models::user::{User, Username};
use crate::models::user_id::UserId;
use crate::use_case::user_use_cases::UserUseCases;

pub struct CreateUserUseCaseInteractor<R> {
    user_repository: R,
}

impl<R> CreateUserUseCaseInteractor<R> {
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }
}

impl<R> UserUseCases for CreateUserUseCaseInteractor<R>
where
    R: UserRepository,
{
    async fn create(&self, name: &str, email: &str) -> Result<UserDto, UsersError> {
        let user_id = UserId::generate();
        let username = match Username::try_from(name) {
            Ok(n) => n,
            Err(e) => return Err(e),
        };

        let user = User::new(&user_id, &username, email, None);

        let _ = self.user_repository.save(&user).await;

        Ok(UserDto::from(user))
    }

    /// add a new partner
    async fn add_partner(
        &self,
        user_id: &UserId,
        partner_id: &UserId,
    ) -> Result<UserDto, UsersError> {
        let (user_data, partner_data) = match try_join!(
            self.user_repository.find_by_id(user_id),
            self.user_repository.find_by_id(partner_id)
        ) {
            Ok((u, p)) => (u, p),
            Err(e) => return Err(e)
        };

        let mut user = match user_data {
            Some(user) => user,
            None => return Err(UsersError::UserNotFind(user_id.id.as_str().into())),
        };

        let mut partner = match partner_data {
            Some(user) => user,
            None => return Err(UsersError::UserNotFind(user_id.id.as_str().into())),
        };

        user.add_partner(partner_id);
        partner.add_partner(user_id);

        let res = try_join!(
            self.user_repository.save(&user),
            self.user_repository.save(&partner)
        );

        if res.is_err() {
            return Err(res.unwrap_err())
        }

        return Ok(UserDto::from(user));
    }
}

#[cfg(test)]
mod add_partner_test {
    use mockall::mock;
    use crate::models::repository::user_repository::MockUserRepository;

    use super::*;

    #[tokio::test]
    async fn add_new_partner() {
        // Arrange
        let mut mock_repo = MockUserRepository::new();
        let user_id = UserId::generate();
        let user_name = Username::try_from("target user").unwrap();
        let user = User::new(&user_id, &user_name, "em@il", None);

        let partner_id = UserId::generate();
        let partner_name = Username::try_from("target name").unwrap();
        let partner = User::new(&partner_id, &partner_name, "email@il", None);

        mock_repo
            .expect_find_by_id()
            .times(1)
            .returning(move |_| Ok(Some((&user).clone())));
        mock_repo
            .expect_find_by_id()
            .times(1)
            .returning(move |_| Ok(Some((&partner).clone())));
        mock_repo
            .expect_save()
            .returning(move |_| Ok(()));

        let use_case = CreateUserUseCaseInteractor::new(mock_repo);

        // Action
        let res_user = use_case.add_partner(&user_id, &partner_id).await.unwrap();

        // Assert
        assert_eq!(res_user.partners.len(), 1);
    }
}
