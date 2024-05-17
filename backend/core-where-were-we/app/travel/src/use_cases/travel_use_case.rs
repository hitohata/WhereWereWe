//! Travel use cases

use crate::dtos::travel::TravelDto;
use crate::errors::errors::TravelError;
use crate::models::repository::travel_repository::TravelRepository;
use crate::models::travel::entity::travel::Travel;
use crate::models::travel::id::travel_id::TravelId;
use crate::models::travel::id::user_id::UserId;

pub trait TravelUseCases {
    /// Create a new travel
    /// The request user is set as one of the travelers.
    async fn create_new_travel(&self, user_id: &str, travel_name: &str, start_date: &str, end_date: Option<&str>) -> Result<TravelDto, TravelError>;
    async fn modify_travel(&self) -> Result<TravelDto, TravelError>;
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
    async fn create_new_travel(&self, user_id: &str, travel_name: &str, start_date: &str, end_date: Option<&str>) -> Result<TravelDto, TravelError> {
        
        let travel_id = TravelId::generate();
        
        let user_id = UserId::try_from(user_id)?;
        
        let travel = Travel::new(&travel_id, travel_name, start_date, end_date, &vec![user_id], None)?;
        
        self.travel_repository.save(&travel).await?;

        Ok(TravelDto::from(&travel))
    }

    async fn modify_travel(&self) -> Result<TravelDto, TravelError> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use std::any::Any;
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
}