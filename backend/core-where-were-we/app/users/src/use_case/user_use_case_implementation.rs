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
    /// create a new user
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

    /// Add a new partner
    /// The partner is added to the user.
    /// The user is also added to the partner's partner.
    async fn add_partner(
        &self,
        user_id: &str,
        partner_id: &str,
    ) -> Result<UserDto, UsersError> {

        let (user_id_struct, partner_id_struct) = match to_user_id_struct(user_id, partner_id) {
            Ok((u, p)) => (u, p),
            Err(e) => return Err(e)
        };

        let (user_data, partner_data) = match try_join!(
            self.user_repository.find_by_id(&user_id_struct),
            self.user_repository.find_by_id(&partner_id_struct),
        ) {
            Ok((u, p)) => (u, p),
            Err(e) => return Err(e)
        };

        let mut user = match user_data {
            Some(user) => user,
            None => return Err(UsersError::UserNotFind(user_id.to_string())),
        };

        let mut partner = match partner_data {
            Some(user) => user,
            None => return Err(UsersError::UserNotFind(partner_id.to_string())),
        };

        user.add_partner(&partner_id_struct);
        partner.add_partner(&user_id_struct);

        let res = try_join!(
            self.user_repository.save(&user),
            self.user_repository.save(&partner)
        );

        match res {
            Ok(_) => Ok(UserDto::from(user)),
            Err(e) => Err(e)
        }
    }

    /// Remove a new partner
    /// The partner is removed from the user.
    /// The user is also removed from the partner's partner.
    async fn remove_partner(&self, user_id: &str, partner_id: &str) -> Result<UserDto, UsersError> {
        let (user_id_struct, partner_id_struct) = match to_user_id_struct(user_id, partner_id) {
            Ok((u, p)) => (u, p),
            Err(e) => return Err(e)
        };

        let (user_data, partner_data) = match try_join!(
            self.user_repository.find_by_id(&user_id_struct),
            self.user_repository.find_by_id(&partner_id_struct),
        ) {
            Ok((u, p)) => (u, p),
            Err(e) => return Err(e)
        };

        let mut user = match user_data {
            Some(user) => user,
            None => return Err(UsersError::UserNotFind(user_id.to_string())),
        };

        let mut partner = match partner_data {
            Some(user) => user,
            None => return Err(UsersError::UserNotFind(partner_id.to_string())),
        };

        user.remove_partner(&partner_id_struct);
        partner.remove_partner(&user_id_struct);


        let res = try_join!(
            self.user_repository.save(&user),
            self.user_repository.save(&partner)
        );

        match res {
            Ok(_) => Ok(UserDto::from(user)),
            Err(e) => Err(e)
        }
    }

    /// update a user's name
    async fn change_name(&self, user_id: &str, new_name: &str) -> Result<UserDto, UsersError> {

        let user_name = Username::try_from(new_name)?;

        let user_id_struct = match UserId::try_from(user_id) {
            Ok(u) => u,
            Err(_) => {
                return Err(UsersError::UsersUseCaseError(format!("Invalid ID is provided {}", user_id)))
            }
        };

        let user_or_option = self.user_repository.find_by_id(&user_id_struct).await?;
        let user = match user_or_option {
            Some(user) => {user},
            None => return Err(UsersError::UserNotFind(user_id.to_string()))
        };

        let updated_name_user = user.update_name(&user_name);

        self.user_repository.save(&updated_name_user).await?;

        Ok(UserDto::from(updated_name_user))
    }
}

fn to_user_id_struct(user_id: &str, partner_id: &str) -> Result<(UserId, UserId), UsersError> {
    let user_id_struct = match UserId::try_from(user_id) {
        Ok(id) => id,
        Err(e) => return Err(e)
    };
    let partner_id_struct = match UserId::try_from(partner_id) {
        Ok(id) => id,
        Err(e) => return Err(e)
    };

    Ok((user_id_struct, partner_id_struct))
}

#[cfg(test)]
mod add_partner_test {
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
        let res_user = use_case.add_partner(user_id.id(), partner_id.id()).await.unwrap();

        // Assert
        assert_eq!(res_user.partners.len(), 1);
    }

    #[tokio::test]
    async fn remove_partner() {
        // Arrange
        let mut mock_repo = MockUserRepository::new();
        let user_id = UserId::generate();
        let partner_id = UserId::generate();

        let user_name = Username::try_from("target user").unwrap();
        let user = User::new(&user_id, &user_name, "em@il", Some(&vec![partner_id.clone()]));

        let partner_name = Username::try_from("target name").unwrap();
        let partner = User::new(&partner_id, &partner_name, "email@il", Some(&vec![user_id.clone()]));

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
        let res_user = use_case.remove_partner(user_id.id(), partner_id.id()).await.unwrap();

        // Assert
        assert_eq!(res_user.partners.len(), 0);
    }
}
