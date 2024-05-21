use crate::dtos::todo::{ToDoDto, ToDoListGroupDto};
use crate::errors::errors::TravelError;
use crate::models::repository::todo_repository::TodoRepository;
use crate::models::repository::travel_repository::TravelRepository;
use crate::models::todo::id::todo_list_group_id::TodoListGroupId;
use crate::models::travel::id::travel_id::TravelId;

pub trait ToDoUseCases {
    /// get a to-do list of the travel
    async fn travel_to_do_list_group(&self, travel_id: &str) -> Result<Vec<ToDoListGroupDto>, TravelError>;
    /// get a to-do list group
    async fn get_todo_list_group(&self, travel_id: &str, to_do_list_group_id: &u32) -> Result<Option<ToDoListGroupDto>, TravelError>;
    /// get a to-do
    async fn get_todo(&self, travel_id: &str, todo_list_group_id: &u32, todo_id: &u32) -> Result<Option<ToDoDto>, TravelError>;
    /// create a new to-do group
    /// The empty to-do is also created
    async fn create_new_todo_group(&self, travel_id: &str, name: &str, tz: Option<isize>) -> Result<ToDoListGroupDto, TravelError>;
    /// create a new to-do
    async fn crate_new_todo(&self, travel_id: &str, todo_list_group_id: &u32, summary: &str, description: Option<&str>, due_date: Option<&str>) -> Result<ToDoDto, TravelError>;
    /// update a to-do list group
    async fn update_todo_list_group(&self, travel_id: &str, todo_list_group_id: &u32, name: &str, tz: Option<isize>) -> Result<ToDoDto, TravelError>;
    /// update a to-do
    async fn update_todo(&self, travel_id: &str, todo_list_group_id: &u32, todo_id: &u32, summary: &str, description: Option<&str>, due_date: Option<&str>) -> Result<ToDoDto, TravelError>;
    /// toggle done section
    async fn toggle_todo_done(&self, travel_id: &str, todo_list_group_id: &u32, todo_id: &u32) -> Result<ToDoDto, TravelError>;
}

pub struct TodoUseCaseInstractor<R, RP> {
    travel_repository: R,
    todo_repository: RP
}

impl <R, RP> TodoUseCaseInstractor<R, RP>
{
    pub fn new(travel_repository: R, todo_repository: RP) -> Self {
        Self {
            travel_repository,
            todo_repository
        }
    }
}

impl<R, RP> ToDoUseCases for TodoUseCaseInstractor<R, RP>
    where R: TravelRepository, RP: TodoRepository
{
    async fn travel_to_do_list_group(&self, travel_id: &str) -> Result<Vec<ToDoListGroupDto>, TravelError> {
        
        let travel_id_struct = match TravelId::try_from(travel_id) {
            Ok(t_id) => t_id,
            Err(e) => return Err(e)
        };
        
        match self.todo_repository.list_todo_list_group(&travel_id_struct).await {
            Ok(todo_list_group) => Ok(todo_list_group.iter().map(|el| ToDoListGroupDto::from(el)).collect()),
            Err(e) => Err(e)
        }
    }

    async fn get_todo_list_group(&self, travel_id: &str, to_do_list_group_id: &u32) -> Result<Option<ToDoListGroupDto>, TravelError> {
        let travel_id_struct = match TravelId::try_from(travel_id) {
            Ok(t) => t,
            Err(e) => return Err(e)
        };
        
        let todo_list_group_id = TodoListGroupId::from(to_do_list_group_id);
        
        match self.todo_repository.find_todo_list_group_by_id(&travel_id_struct, &todo_list_group_id).await {
            Ok(todo) => {
                match todo {
                    Some(t) => Ok(Some(ToDoListGroupDto::from(&t))),
                    None => Ok(None)
                }
            },
            Err(e) => Err(e)
        }
    }

    async fn get_todo(&self, travel_id: &str, todo_list_group_id: &u32, todo_id: &u32) -> Result<Option<ToDoDto>, TravelError> {
        todo!()
    }

    async fn create_new_todo_group(&self, travel_id: &str, name: &str, tz: Option<isize>) -> Result<ToDoListGroupDto, TravelError> {
        todo!()
    }

    async fn crate_new_todo(&self, travel_id: &str, todo_list_group_id: &u32, summary: &str, description: Option<&str>, due_date: Option<&str>) -> Result<ToDoDto, TravelError> {
        todo!()
    }

    async fn update_todo_list_group(&self, travel_id: &str, todo_list_group_id: &u32, name: &str, tz: Option<isize>) -> Result<ToDoDto, TravelError> {
        todo!()
    }

    async fn update_todo(&self, travel_id: &str, todo_list_group_id: &u32, todo_id: &u32, summary: &str, description: Option<&str>, due_date: Option<&str>) -> Result<ToDoDto, TravelError> {
        todo!()
    }

    async fn toggle_todo_done(&self, travel_id: &str, todo_list_group_id: &u32, todo_id: &u32) -> Result<ToDoDto, TravelError> {
        todo!()
    }
}