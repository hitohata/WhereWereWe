//! user's model
use crate::errors::errors::UsersError;
use crate::models::user_id::UserId;

#[derive(Debug, Clone, Eq, PartialEq)]
struct User {
    /// User ID
    id: UserId,
    // Username
    name: Username,
    email: String
}

impl User {
    pub fn new(id: &UserId, name: &Username, email: &str) -> Self {
        Self {
            id: id.to_owned(),
            name: name.to_owned(),
            email: email.to_string()
        }
    }

    pub fn update_name(&mut self, name: &Username) {
        self.name = name.to_owned()
    }
}

/// Username must be grater than 0 and less than equal 255.
#[derive(Debug, Clone, Eq, PartialEq)]
struct Username {
    // must be 0 < name <= 255
    name: String
}

impl Username {

    pub fn name(&self) -> String {
        (&self.name).to_string()
    }

    // the username must be 0 < name <= 255.
    pub fn new(name: &str) -> Result<Self, UsersError> {
        let name_len = name.len();

        if 255 < name_len {
            Err(UsersError::UsernameError("The name length must be less than 255 characters.".to_string()))
        } else if name_len < 1 {
            Err(UsersError::UsernameError("The name length must be grater than 0.".to_string()))
        } else {
            Ok(Self {
                name: name.to_string()
            })
        }
    }
}

#[cfg(test)]
mod user_test {
    use super::*;

    #[test]
    fn test_change_name() {
        let mut user = User::new(&UserId::generate(), &Username::new("name").unwrap(), "");
        user.update_name(&Username::new("name2").unwrap());
        assert_eq!(user.name.name(), "name2");
    }
}

#[cfg(test)]
mod username_test {
    use super::*;

    #[test]
    fn test_new_name() {
        let name = Username::new("name").unwrap().name;
        assert_eq!(name, "name");
    }

    #[test]
    fn test_name_too_short() {
        let name = Username::new("");
        assert!(name.is_err());
        assert_eq!(name.unwrap_err().to_string(), "[UsernameError]: The name length must be grater than 0.".to_string());
    }

    #[test]
    fn test_name_too_long() {
        let name = Username::new(&vec!["a"; 256].join(""));
        assert!(name.is_err());
        assert_eq!(name.unwrap_err().to_string(), "[UsernameError]: The name length must be less than 255 characters.".to_string());
    }
}