//! The repository trait of the todo

use crate::errors::errors::TravelError;
use crate::models::todo::entity::todo::Todo;
use crate::models::todo::entity::todo_group::TodoListGroup;
use crate::models::todo::id::todo_id::TodoId;
use crate::models::todo::id::todo_list_group_id::TodoListGroupId;
use crate::models::travel::id::travel_id::TravelId;

trait TodoRepository {
    fn find_todo_group_by_id(travel_id: &TravelId, todo_list_group_id: &TodoListGroupId) -> Result<TodoListGroup, TravelError>;
    fn save_todo_group(todo_group: &TodoListGroup) -> Result<(), TravelError>;
    fn find_todo_by_id(travel_id: &TravelId, todo_list_group_id: &TodoListGroupId, todo: &TodoId) -> Result<Todo, TravelError>;
    fn save_todo(todo: &Todo) -> Result<(), TravelError>;
}