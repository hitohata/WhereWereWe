//! todo ID
//! This is auto increment number
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TodoId {
    id: u32
}

impl From<&u32> for TodoId {
    fn from(value: &u32) -> Self {
        Self { id: value.into() }
    }
}