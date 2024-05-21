use crate::dtos::todo::{ToDoDto, ToDoListGroupDto};
use crate::errors::errors::TravelError;

pub trait ToDoUseCases {
    /// get a to-do list of the travel
    async fn travel_to_do_list_group(&self, travel_id: &str) -> Result<Vec<ToDoListGroupDto>, TravelError>;
    /// get a to-do list group
    async fn get_todo_list_group(&self, travel_id: &str, to_do_list_group_id: &str) -> Result<Option<ToDoListGroupDto>, TravelError>;
    /// get a to-do
    async fn get_todo(&self, travel_id: &str, todo_list_group_id: &str, todo_id: &str) -> Result<Option<ToDoDto>, TravelError>;
    /// create a new to-do group
    /// The empty to-do is also created
    async fn create_new_todo_group(&self, travel_id: &str, name: &str, tz: Option<isize>) -> Result<ToDoListGroupDto, TravelError>;
    /// create a new to-do
    async fn crate_new_todo(&self, travel_id: &str, todo_list_group_id: &str, summary: &str, description: Option<&str>, due_date: Option<&str>) -> Result<ToDoDto, TravelError>;
    /// update a to-do list group
    async fn update_todo_list_group(&self, travel_id: &str, todo_list_group_id: &str, name: &str, tz: Option<isize>) -> Result<ToDoDto, TravelError>;
    /// update a to-do
    async fn update_todo(&self, travel_id: &str, todo_list_group_id: &str, todo_id: &str, summary: &str, description: Option<&str>, due_date: Option<&str>) -> Result<ToDoDto, TravelError>;
    /// toggle done section
    async fn toggle_todo_done(&self, travel_id: &str, todo_list_group_id: &str, todo_id: &str) -> Result<ToDoDto, TravelError>;
}