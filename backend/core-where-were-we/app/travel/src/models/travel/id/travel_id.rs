//! Travel ID
use uuid::Uuid;
use crate::errors::errors::TravelError;

/// travel ID
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub (crate) struct TravelId {
    id: String
}

impl TravelId {
    // Generate a new User ID.
    pub fn generate() -> Self {
        Self {
            id: Uuid::new_v4().to_string()
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

impl TryFrom<&str> for TravelId {
    type Error = TravelError;
    /// The argument is user ID that must be UUID.
    /// If you don't provide a valid ID, this function returns errors.
    fn try_from(id: &str) -> Result<Self, TravelError> {
        match Uuid::try_parse(id) {
            Ok(id_from_string) => {
                Ok(Self {
                    id: id_from_string.to_string()
                })
            }
            Err(_) => Err(TravelError::DomainError("Invalid ID is provided.".to_string()))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eq() {
        let travel_id = TravelId::generate();
        let clone_id = travel_id.clone();
        assert_eq!(travel_id, clone_id);
    }

}