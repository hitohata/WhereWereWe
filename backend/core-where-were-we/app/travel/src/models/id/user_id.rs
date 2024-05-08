//! user's ID
//! This is reference only,
use uuid::Uuid;
use crate::errors::errors::TravelError;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct UserId {
    id: String
}

impl TryFrom<&str> for UserId {
    type Error = TravelError;

    fn try_from(value: &str) -> Result<Self, TravelError> {
        match Uuid::try_parse(value) {
            Ok(id) => Ok(Self { id: id.to_string() }),
            Err(_) => Err(TravelError::DomainError("Invalid User ID was provided.".to_string()))
        }
    }
}

