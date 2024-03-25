use ulid::Ulid;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct UserId {
    id: String
}

impl UserId {
    pub fn id(&self) -> String {
        (&self.id).into()
    }

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