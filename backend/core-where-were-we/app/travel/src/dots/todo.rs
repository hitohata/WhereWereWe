//! To-do DTO
use serde::{Serialize, Deserialize};
use chrono;
use crate::models::todo::entity::todo::Todo;
use crate::models::todo::entity::todo_group::TodoListGroup;

/// To-do list group DTO
#[derive(Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ToDoListGroupDto {
    pub travel_id: String,
    pub todo_list_group_id: usize,
    pub group_name: String,
    pub todo: Vec<ToDoDto>,
    pub tz: Option<usize>
}

/// To-do DTO
#[derive(Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ToDoDto {
    pub todo_id: usize,
    pub summary: String,
    pub description: Option<String>,
    pub due_date: Option<String>,
    pub done: bool
}

impl From<&TodoListGroup> for ToDoListGroupDto {
    /// Convert the To-do list group into DTO
    fn from(todo_list_group: &TodoListGroup) -> Self {
        Self {
            travel_id: todo_list_group.travel_id().id().to_string(),
            todo_list_group_id: todo_list_group.todo_list_group_id().id().to_owned() as usize,
            group_name: todo_list_group.group_name().to_string(),
            todo: todo_list_group.todo().iter().map(|todo| ToDoDto::from(todo)).collect(),
            tz: match todo_list_group.tz() {
                Some(tz) => Some(tz as usize),
                None => None
            }
        }
    }
} 

impl From<&Todo> for ToDoDto {
    /// Convert the To-do struct into DTO.
    fn from(todo: &Todo) -> Self {
        Self {
            todo_id: todo.todo_id().id().to_owned() as usize,
            summary: todo.summary().to_string(),
            description: match todo.description() {
                Some(desc) => Some(desc.to_string()),
                None => None
            },
            due_date: match todo.due_date() {
                Some(data) => {
                    let dt = chrono::DateTime::from_timestamp_nanos(data.to_owned());
                    Some(dt.to_rfc3339()) // convert the timestamp into the String
                },
                None => None
            },
            done: todo.done()
        }
    }
}