//! The repository trait of the to-do

use mockall::*;
use crate::errors::errors::TravelError;
use crate::models::todo::entity::todo::Todo;
use crate::models::todo::entity::todo_group::TodoListGroup;
use crate::models::todo::id::todo_id::TodoId;
use crate::models::todo::id::todo_list_group_id::TodoListGroupId;
use crate::models::travel::id::travel_id::TravelId;

#[automock]
pub trait TodoRepository {
    async fn find_todo_list_group_by_id(&self, travel_id: &TravelId, todo_list_group_id: &TodoListGroupId) -> Result<Option<TodoListGroup>, TravelError>;
    async fn list_todo_list_group(&self, travel_id: &TravelId) -> Result<Vec<TodoListGroup>, TravelError>;
    async fn save_todo_list_group(&self, todo_group: &TodoListGroup) -> Result<(), TravelError>;
    async fn find_todo_by_id(&self, travel_id: &TravelId, todo_list_group_id: &TodoListGroupId, todo: &TodoId) -> Result<Option<Todo>, TravelError>;
    async fn list_todo(&self, travel_id: &TravelId, todo_list_group_id: &TodoListGroupId) -> Result<Vec<Todo>, TravelError>;
    async fn save_todo(&self, travel_id: &TravelId, todo_list_group_id: &TodoListGroupId, todo: &Todo) -> Result<(), TravelError>;
}