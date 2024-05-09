//! todo ID
//! This is auto increment number
pub struct TodoId {
    id: u32
}

impl TodoId {
    pub fn new(id: &u32) -> Self {
        Self { id: id.into() }
    }
}