//! todo list group ID
//! This is auto increment number
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TodoListGroupId {
    id: u32
}

impl From<&u32> for TodoListGroupId {
    fn from(value: &u32) -> Self {
        Self { id: value.to_owned() }
    }
}
