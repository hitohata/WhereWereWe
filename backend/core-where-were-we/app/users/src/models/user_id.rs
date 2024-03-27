//! The User ID
use uuid::Uuid;
use crate::errors::errors::UsersError;

/// User ID consists of an ID only that is UUID
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub (crate) struct UserId {
    pub (crate) id: String
}

impl UserId {
    /// The argument is user ID that must be UUID.
    /// If you don't provide a valid ID, this function returns error.
    pub fn create(id: &str) -> Result<Self, UsersError> {
        match Uuid::try_parse(id) {
            Ok(id_from_string) => {
                Ok(Self {
                    id: id_from_string.to_string()
                })
            }
            Err(_) => Err(UsersError::InvalidUUID)
        }
    }

    // Generate a new User ID.
    pub fn generate() -> Self {
        Self {
            // id: Ulid::new().to_string()
            id: Uuid::new_v4().to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eq() {
        let user_id = UserId::generate();
        let clone_id = user_id.clone();
        assert_eq!(user_id, clone_id);
    }

}