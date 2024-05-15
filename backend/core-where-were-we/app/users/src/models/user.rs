//! user's model
use crate::errors::errors::UsersError;
use crate::models::user_id::UserId;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct User {
    /// User ID
    id: UserId,
    /// Username
    name: Username,
    email: String,
    /// the partners
    /// if there is no partners, this value will be an empty vector.
    partners: Vec<UserId>
}

impl User {
    pub (crate) fn new(id: &UserId, name: &Username, email: &str, partners: Option<&Vec<UserId>>) -> Self {
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
    
    pub fn id(&self) -> &UserId { 
        &self.id
    }
    
    pub fn name(&self) -> &str {
        &self.name.name
    }
    
    pub fn email(&self) -> &str {
        &self.email
    } 
    
    pub fn partners(&self) -> &Vec<UserId> {
        &self.partners
    }

    // change the username
    pub fn update_name(mut self, name: &Username) -> Self {
        self.name = name.to_owned();
        self
    }

    // add a new partner
    pub fn add_partner(&mut self, partner_id: &UserId) {
        // if the required partner is not existing in the partner, append it.
        if !self.partners.iter().any(|id| id.eq(&partner_id)) {
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
pub struct Username {
    // must be 0 < name <= 255
    name: String
}

impl Username {
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl TryFrom<&str> for Username {
    type Error = UsersError;

    fn try_from(name: &str) -> Result<Self, UsersError> {
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
        let user = User::new(&UserId::generate(), &Username::try_from("name").unwrap(), "", None);
        let updated = user.update_name(&Username::try_from("name2").unwrap());
        assert_eq!(updated.name.name, "name2");
    }

    #[test]
    fn test_add_a_new_partner() {
        // Arrange
        let user_id = UserId::generate();
        let mut user = User::new(&UserId::generate(), &Username::try_from("name").unwrap(), "", None);

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
        let mut user = User::new(&UserId::generate(), &Username::try_from("name").unwrap(), "", Some(&partners));

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
        let name = Username::try_from("name").unwrap().name;
        assert_eq!(name, "name");
    }

    #[test]
    fn test_name_too_short() {
        let name = Username::try_from("");
        assert!(name.is_err());
        assert_eq!(name.unwrap_err().to_string(), "The name length must be grater than 0.".to_string());
    }

    #[test]
    fn test_name_too_long() {
        let name = Username::try_from(vec!["a"; 256].join("").as_str());
        assert!(name.is_err());
        assert_eq!(name.unwrap_err().to_string(), "The name length must be less than 255 characters.".to_string());
    }
}