//! The User ID
use uuid::Uuid;
use crate::errors::errors::UsersError;

/// User ID consists of an ID only that is UUID
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct UserId {
    pub (crate) id: String
}

impl UserId {

    pub fn id(&self) -> &str {
        &self.id
    }

    // Generate a new User ID.
    pub (crate) fn generate() -> Self {
        Self {
            // id: Ulid::new().to_string()
            id: Uuid::new_v4().to_string()
        }
    }
}

impl TryFrom<&str> for UserId {
    type Error = UsersError;
    /// The argument is user ID that must be UUID.
    /// If you don't provide a valid ID, this function returns errors.
    fn try_from(id: &str) -> Result<Self, UsersError> {
        match Uuid::try_parse(id) {
            Ok(id_from_string) => {
                Ok(Self {
                    id: id_from_string.to_string()
                })
            }
            Err(_) => Err(UsersError::InvalidUUID)
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