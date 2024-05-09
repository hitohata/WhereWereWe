//! todo list group ID
//! This is auto increment number
pub struct TodoListGroupId {
    id: u32
}

impl TodoListGroupId {
    pub fn new(id: &u32) -> Self {
        Self { id: id.into() }
    }
}