//! This is the collection of the todo list
use crate::models::todo::entity::todo::Todo;
use crate::models::todo::id::todo_id::TodoId;
use crate::models::todo::id::todo_list_group_id::TodoListGroupId;
use crate::models::travel::id::travel_id::TravelId;

pub struct TodoListGroup {
    travel_id: TravelId,
    todo_group_id: TodoListGroupId,
    group_name: String,
    todo: Vec<Todo>
}

impl TodoListGroup {
    pub fn new(travel_id: &TravelId, todo_group_id: &TodoListGroupId, group_name: &str, todo: Vec<Todo>) -> Self {
        Self {
            travel_id: travel_id.to_owned(),
            todo_group_id: todo_group_id.to_owned(),
            group_name: group_name.to_owned(),
            todo
        }
    }

    /// remove a todo from this collection
    pub fn remove_todo(&mut self, todo_id: &TodoId) {
        if let Some(index) = self.todo.iter().position(|x| x.todo_id().eq(todo_id)) {
            self.todo.remove(index);
        }
    }

    /// add a new todo to this collection
    pub fn add_todo(&mut self, todo: &Todo) {
        self.todo.push(todo.clone());
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
        let todo = Todo::new(&TodoId::from(&1u32), "summary", None, None).unwrap();
        let mut todo_list_group = TodoListGroup::new(&travel_id, &todo_group_id, "name", vec![]);

        // Act
        todo_list_group.add_todo(&todo);

        // Assert
        assert_eq!(todo_list_group.todo.len(), 1);
    }

    #[test]
    fn test_remove_todo() {
        // Arrange
        let travel_id = TravelId::generate();
        let todo_group_id = TodoListGroupId::from(&1);
        let todo = Todo::new(&TodoId::from(&1u32), "summary", None, None).unwrap();
        let mut todo_list_group = TodoListGroup::new(&travel_id, &todo_group_id, "name", vec![todo]);

        // Act
        todo_list_group.remove_todo(&TodoId::from(&1u32));

        // Assert
        assert_eq!(todo_list_group.todo.len(), 0);
    }
}