use crate::models::user_id::UserId;

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
            email: name.to_string()
        }
    }

    pub fn update_name(&mut self, name: &Username) {
        self.name = name.to_owned()
    }
}

/// Username must be grater than 0 and less than equal 255.
#[derive(Debug, Clone, Eq, PartialEq)]
struct Username {
    name: String
}

impl Username {
    // TODO: implement validation
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string()
        }
    }
}

// #[cfg(test)]
