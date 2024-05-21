//! This is the collection of the to do list
use crate::errors::errors::TravelError;
use crate::models::todo::entity::todo::Todo;
use crate::models::todo::id::todo_id::TodoId;
use crate::models::todo::id::todo_list_group_id::TodoListGroupId;
use crate::models::travel::id::travel_id::TravelId;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TodoListGroup {
    /// travel ID
    travel_id: TravelId,
    /// to-do group ID
    todo_list_group_id: TodoListGroupId,
    /// this to-do group name
    group_name: String,
    /// The collection of the to-do rest
    todo: Vec<Todo>,
    /// time offset from the UTC
    tz: Option<i32>
}

impl TodoListGroup {
    pub fn new(travel_id: &TravelId, todo_group_id: &TodoListGroupId, group_name: &str, todo: Vec<Todo>, tz: Option<i32>) -> Result<Self, TravelError> {
        
        if group_name.len() < 1 {
            return Err(TravelError::DomainError("The travel name cannot be empty".to_string()))
        }
        
        if 200 <= group_name.len() {
            return Err(TravelError::DomainError("The travel name cannot be grater than or equal to 200".to_string()))
        }
        
        Ok(Self {
            travel_id: travel_id.to_owned(),
            todo_list_group_id: todo_group_id.to_owned(),
            group_name: group_name.to_owned(),
            todo,
            tz
        })
    }
    
    pub fn travel_id(&self) -> &TravelId {
        &self.travel_id
    }
    
    pub fn todo_list_group_id(&self) -> &TodoListGroupId {
        &self.todo_list_group_id
    }
    
    pub fn group_name(&self) -> &str {
        &self.group_name
    }
    
    pub fn todo(&self) -> &Vec<Todo> {
        &self.todo
    } 
    
    pub fn tz(&self) -> Option<i32> {
        match self.tz {
            Some(t) => Some(t),
            None => None
        }
    }

    /// remove a to-do from this collection
    pub fn remove_todo(mut self, todo_id: &TodoId) -> Self {
        if let Some(index) = self.todo.iter().position(|x| x.todo_id().eq(todo_id)) {
            self.todo.remove(index);
        }
        self
    }

    /// add a new to do to this collection
    pub fn add_todo(mut self, todo: &Todo) -> Self {
        self.todo.push(todo.clone());
        self
    }
    
    /// update the to-do list group by creating a new struct
    pub fn update(&self, group_name: &str, tz: Option<i32>) -> Result<Self, TravelError> {
        TodoListGroup::new(
            &self.travel_id,
            &self.todo_list_group_id,
            group_name,
            self.todo.to_owned(),
            tz
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_todo() {
        // Arrange
        let travel_id = TravelId::generate();
        let todo_group_id = TodoListGroupId::from(&1);
        let todo = Todo::new(&TodoId::from(&1u32), "summary", None, None, None).unwrap();
        let todo_list_group = TodoListGroup::new(&travel_id, &todo_group_id, "name", vec![], None).unwrap();

        // Act
        let updated = todo_list_group.add_todo(&todo);

        // Assert
        assert_eq!(updated.todo.len(), 1);
    }

    #[test]
    fn test_remove_todo() {
        // Arrange
        let travel_id = TravelId::generate();
        let todo_group_id = TodoListGroupId::from(&1);
        let todo = Todo::new(&TodoId::from(&1u32), "summary", None, Some(42), None).unwrap();
        let todo_list_group = TodoListGroup::new(&travel_id, &todo_group_id, "name", vec![todo], Some(9)).unwrap();
        

        // Act
        let updated = todo_list_group.remove_todo(&TodoId::from(&1u32));

        // Assert
        assert_eq!(updated.todo.len(), 0);
    }
}