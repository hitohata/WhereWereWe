//! user's model
use crate::errors::errors::UsersError;
use crate::models::user_id::UserId;

#[derive(Debug, Clone, Eq, PartialEq)]
pub (crate) struct User {
    /// User ID
    pub (crate) id: UserId,
    /// Username
    pub (crate) name: Username,
    pub (crate) email: String,
    /// the partners
    /// if there is no partners, this value will be an empty vector.
    pub (crate) partners: Vec<UserId>
}

impl User {
    pub fn new(id: &UserId, name: &Username, email: &str, partners: Option<&Vec<UserId>>) -> Self {
        Self {
            id: id.to_owned(),
            name: name.to_owned(),
            email: email.to_string(),
            partners: match partners {
                Some(val) => val.to_owned(),
                None => Vec::new()
            }
        }
    }

    // change the username
    pub fn update_name(&mut self, name: &Username) {
        self.name = name.to_owned()
    }

    // add a new partner
    pub fn add_partner(&mut self, partner_id: &UserId) {
        // if the required partner is not existing in the partner, append it.
        if self.partners.iter().find(|id| id.eq(&partner_id)).is_none() {
            self.partners.push(partner_id.to_owned())
        }
    }

    // remove a partner
    pub fn remove_partner(&mut self, partner_id: &UserId) {
        self.partners = self.partners.iter().filter(|id| !id.eq(&partner_id)).cloned().collect::<Vec<UserId>>();
    }
}

/// Username must be grater than 0 and less than equal 255.
#[derive(Debug, Clone, Eq, PartialEq)]
pub (crate) struct Username {
    // must be 0 < name <= 255
    pub (crate) name: String
}

impl Username {
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
        let mut user = User::new(&UserId::generate(), &Username::new("name").unwrap(), "", None);
        user.update_name(&Username::new("name2").unwrap());
        assert_eq!(user.name.name(), "name2");
    }

    #[test]
    fn test_add_a_new_partner() {
        // Arrange
        let user_id = UserId::generate();
        let mut user = User::new(&UserId::generate(), &Username::new("name").unwrap(), "", None);

        // Act
        user.add_partner(&user_id);

        // Assert
        assert_eq!(user.partners.len(), 1); // user_id2 is removed
        assert_eq!(user.partners, vec![user_id]);
    }

    #[test]
    fn test_remove_partner() {
        // Arrange
        let user_id = UserId::generate();
        let user_id2 = UserId::generate();
        let partners = vec![user_id.clone(), user_id2.clone()];
        let mut user = User::new(&UserId::generate(), &Username::new("name").unwrap(), "", Some(&partners));

        // Act
        user.remove_partner(&user_id2);

        // Assert
        assert_eq!(user.partners.len(), 1); // user_id2 is removed
        assert_eq!(user.partners, vec![user_id]);
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