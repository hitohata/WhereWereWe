//! Travel use cases

use crate::dtos::travel::TravelDto;
use crate::errors::errors::TravelError;
use crate::models::repository::travel_repository::TravelRepository;
use crate::models::travel::entity::travel::Travel;
use crate::models::travel::id::travel_id::TravelId;
use crate::models::travel::id::user_id::UserId;

pub trait TravelUseCases {
    /// get a travel
    async fn get_travel(&self, travel_id: &str, user_id: &str) -> Result<Option<TravelDto>, TravelError>;
    /// Create a new travel
    /// The request user is set as one of the travelers.
    async fn create_new_travel(&self, user_id: &str, travel_name: &str, start_date: &str, end_date: Option<&str>) -> Result<TravelDto, TravelError>;
    /// Only travelers can modify the travel information.
    /// When the involved user tries to modify, its attempt will be rejected. 
    async fn modify_travel(&self, travel_id: &str, travel_name: &str, start_date: &str, end_date: Option<&str>, travelers: &Vec<&str>, involved_users: &Vec<&str>, user_id: &str) -> Result<TravelDto, TravelError>;
}

pub struct TravelUseCasesInteractor<R> {
    travel_repository: R,
}

impl<R> TravelUseCasesInteractor<R> {
    pub fn new(travel_repository: R) -> Self {
        Self { travel_repository }
    }
}

impl<R> TravelUseCases for TravelUseCasesInteractor<R>
    where R: TravelRepository
{
    async fn get_travel(&self, travel_id: &str, user_id: &str) -> Result<Option<TravelDto>, TravelError> {
        let travel_id_struct = TravelId::try_from(travel_id)?;
        let user_id_struct = UserId::try_from(user_id)?;
        let travel = self.travel_repository.find_by_id(&travel_id_struct).await?;
        
        Ok(match travel {
            Some(t) => {
                match t.is_related_parties(&user_id_struct) {
                    true => Some(TravelDto::from(&t)),
                    false => return Err(TravelError::AuthenticationError)
                }
            },
            None => None
        })
    }
    
    async fn create_new_travel(&self, user_id: &str, travel_name: &str, start_date: &str, end_date: Option<&str>) -> Result<TravelDto, TravelError> {
        
        let travel_id = TravelId::generate();
        
        let user_id = UserId::try_from(user_id)?;
        
        let travel = Travel::new(&travel_id, travel_name, start_date, end_date, &vec![user_id], None)?;
        
        self.travel_repository.save(&travel).await?;

        Ok(TravelDto::from(&travel))
    }

    async fn modify_travel(&self, travel_id: &str, travel_name: &str, start_date: &str, end_date: Option<&str>, travelers: &Vec<&str>, involved_users: &Vec<&str>, user_id: &str) -> Result<TravelDto, TravelError> {
        
        let travel_id_struct = TravelId::try_from(travel_id)?;
        let user_id_struct = UserId::try_from(user_id)?;
        
        let travel_data = self.travel_repository.find_by_id(&travel_id_struct).await?;
        
        // check validation
        match travel_data {
            Some(travel) => {
                if !travel.is_traveler(&user_id_struct) {
                    return Err(TravelError::AuthenticationError)
                }
            },
            None => return Err(TravelError::NotFound("Travel data".to_string()))
        }
        
        let mut travelers_vec: Vec<UserId> = Vec::new();
        let mut involved_users_vec = Vec::new();
        
        for traveler in travelers {
            match UserId::try_from(*traveler) {
                Ok(u_id) => travelers_vec.push(u_id),
                Err(e) => return Err(e)
            }
        };

        for involved_user in involved_users {
            match UserId::try_from(*involved_user) {
                Ok(u_id) => involved_users_vec.push(u_id),
                Err(e) => return Err(e)
            }
        };
        
        let travel = Travel::new(
            &travel_id_struct,
            travel_name,
            start_date,
            end_date,
            &travelers_vec,
            Some(&involved_users_vec)
        )?;

        self.travel_repository.save(&travel).await?;
        
        Ok(TravelDto::from(&travel))
    }
}

#[cfg(test)]
mod test {
    use crate::models::repository::travel_repository::MockTravelRepository;
    use super::*;
    
    #[tokio::test]
    async fn test_create_new_travel() {
        
        // Arrange
        let mut mock_repo = MockTravelRepository::new();
        let user_id = TravelId::generate(); // user ID and the travel ID both are the UUID.
        
        mock_repo
            .expect_save()
            .returning(move |_| Ok(()));
        
        let use_case = TravelUseCasesInteractor::new(mock_repo);
        
        // Act
        let result = use_case.create_new_travel(user_id.id(), "travel name", "2024-05-12T06:28:49+00:00", Some("2024-05-13T06:28:49+00:00")).await;
        
        // Assert
        assert!(result.is_ok());
        
        let travel_dto = result.unwrap();
        assert_eq!(travel_dto.name, "travel name");
        assert_eq!(travel_dto.travelers, vec![user_id.id()]);
        assert_eq!(travel_dto.involved_users.len(), 0);
    }

    #[tokio::test]
    async fn test_modify_travel() {

        // Arrange
        let mut mock_repo = MockTravelRepository::new();
        let user_id = UserId::try_from(TravelId::generate().id()).unwrap(); // user ID and the travel ID both are the UUID.
        
        let travel_id = TravelId::generate();
        
        let travelers = vec![user_id.clone()];
        
        let travel = Travel::new(&travel_id, "travel name","2024-05-12T06:28:49+00:00", Some("2024-05-13T06:28:49+00:00") , &travelers, None).unwrap();
        
        mock_repo
            .expect_find_by_id()
            .returning(move |_| Ok(Some(travel.clone())));
        
        mock_repo
            .expect_is_users_travel()
            .returning(move |_, _| Ok(true));
        
        mock_repo
            .expect_save()
            .returning(move |_| Ok(()));

        let use_case = TravelUseCasesInteractor::new(mock_repo);

        let travel_id = TravelId::try_from(travel_id.id()).unwrap();

        // Act
        let result = use_case.modify_travel(travel_id.id(), "updated", "2024-05-12T06:28:49+00:00", Some("2024-05-13T06:28:49+00:00") , &travelers.iter().map(|t| t.id()).collect(), &vec![], user_id.id()).await;
        
        // Assert
        assert!(result.is_ok());
        
        let travel_dto = result.unwrap();
        assert_eq!(travel_dto.name, "updated");
    }

    #[tokio::test]
    async fn test_modify_travel_if_the_travel_is_not_found() {

        // Arrange

        let user_id = UserId::try_from(TravelId::generate().id()).unwrap(); // user ID and the travel ID both are the UUID.
        let mut mock_repo = MockTravelRepository::new();
        let travel_id = TravelId::generate();

        mock_repo
            .expect_find_by_id()
            .returning(move |_| Ok(None));
        
        mock_repo
            .expect_save()
            .returning(move |_| Ok(()));

        let use_case = TravelUseCasesInteractor::new(mock_repo);

        // Act
        let result = use_case.modify_travel(travel_id.id(), "updated", "2024-05-12T06:28:49+00:00", Some("2024-05-13T06:28:49+00:00") , &vec![], &vec![], user_id.id()).await;

        // Assert
        assert!(result.is_err());
    }


    #[tokio::test]
    async fn test_modify_travel_user_is_not_in_the_ravel() {

        // Arrange
        let mut mock_repo = MockTravelRepository::new();
        let user_id = UserId::try_from(TravelId::generate().id()).unwrap(); // user ID and the travel ID both are the UUID.

        let travel_id = TravelId::generate();

        let travelers = vec![user_id.clone()];

        let travel = Travel::new(&travel_id, "travel name","2024-05-12T06:28:49+00:00", Some("2024-05-13T06:28:49+00:00") , &travelers, None).unwrap();

        mock_repo
            .expect_find_by_id()
            .returning(move |_| Ok(Some(travel.clone())));

        mock_repo
            .expect_save()
            .returning(move |_| Ok(()));

        let use_case = TravelUseCasesInteractor::new(mock_repo);

        let travel_id = TravelId::try_from(travel_id.id()).unwrap();

        // Act
        let result = use_case.modify_travel(travel_id.id(), "updated", "2024-05-12T06:28:49+00:00", Some("2024-05-13T06:28:49+00:00") , &travelers.iter().map(|t| t.id()).collect(), &vec![], user_id.id()).await;

        // Assert
        assert!(result.is_ok());

        let travel_dto = result.unwrap();
        assert_eq!(travel_dto.name, "updated");
    }
}