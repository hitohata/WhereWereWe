//! The User ID
use ulid::Ulid;

/// User ID consists of an ID only that is UUID
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct UserId {
    id: String
}

impl UserId {
    pub fn id(&self) -> String {
        (&self.id).into()
    }

    /// The argument is user ID that must be UUID.
    /// If you don't provide a valid ID, this function returns error.
    pub fn create(id: &str) -> Result<Self, ulid::DecodeError> {
        match Ulid::from_string(id) {
            Ok(id_from_string) => {
                Ok(Self {
                    id: id_from_string.to_string()
                })
            }
            Err(e) => Err(e)
        }
    }

    // Generate a new User ID.
    pub fn generate() -> Self {
        Self {
            id: Ulid::new().to_string()
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