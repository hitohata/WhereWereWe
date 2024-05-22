use crate::dtos::todo::{ToDoDto, ToDoListGroupDto};
use crate::errors::errors::TravelError;
use crate::models::repository::todo_repository::TodoRepository;
use crate::models::repository::travel_repository::TravelRepository;
use crate::models::todo::entity::todo::Todo;
use crate::models::todo::entity::todo_list_group::TodoListGroup;
use crate::models::todo::id::todo_id::TodoId;
use crate::models::todo::id::todo_list_group_id::TodoListGroupId;
use crate::models::travel::id::travel_id::TravelId;
use crate::models::travel::id::user_id::UserId;
use crate::service::todo_id_service::TodoIdService;

pub trait ToDoUseCases {
    /// get a to-do list of the travel
    async fn travel_to_do_list_group(&self, travel_id: &str) -> Result<Vec<ToDoListGroupDto>, TravelError>;
    /// get a to-do list group
    async fn get_todo_list_group(&self, travel_id: &str, to_do_list_group_id: &u32) -> Result<Option<ToDoListGroupDto>, TravelError>;
    /// get a to-do
    async fn get_todo(&self, travel_id: &str, todo_list_group_id: &u32, todo_id: &u32) -> Result<Option<ToDoDto>, TravelError>;
    /// create a new to-do group
    /// The empty to-do is also created
    async fn create_new_todo_list_group(&self, user_id: &str, travel_id: &str, name: &str, tz: Option<i64>) -> Result<ToDoListGroupDto, TravelError>;
    /// create a new to-do
    async fn crate_new_todo(&self, user_id: &str, travel_id: &str, todo_list_group_id: &u32, summary: &str, description: Option<&str>, due_date: Option<&str>) -> Result<ToDoDto, TravelError>;
    /// update a to-do list group
    async fn update_todo_list_group(&self, user_id: &str, travel_id: &str, todo_list_group_id: &u32, name: &str, tz: Option<i32>) -> Result<ToDoListGroupDto, TravelError>;
    /// update a to-do
    async fn update_todo(&self, user_id: &str, travel_id: &str, todo_list_group_id: &u32, todo_id: &u32, summary: &str, description: Option<&str>, due_date: Option<&str>) -> Result<ToDoDto, TravelError>;
    /// toggle done section
    async fn toggle_todo_done(&self, user_id: &str, travel_id: &str, todo_list_group_id: &u32, todo_id: &u32) -> Result<ToDoDto, TravelError>;
}

pub struct TodoUseCaseInstractor<R, RP, S> {
    travel_repository: R,
    todo_repository: RP,
    todo_id_service: S
}


impl<R, RP, S> TodoUseCaseInstractor<R, RP, S>
    where R: TravelRepository, RP: TodoRepository, S: TodoIdService {

    /// check if the user belong to a travel
    async fn check_authentication(&self, user_id: &str, travel_id: &str) -> Result<(), TravelError> {
        let user_id = UserId::try_from(user_id)?;
        let travel_id = TravelId::try_from(travel_id)?;

        let travel = self.travel_repository.find_by_id(&travel_id).await?;

        match travel {
            Some(t) => {
                if !t.is_related_parties(&user_id) {
                    return Err(TravelError::AuthenticationError);
                };
                Ok(())
            },
            None => return Err(TravelError::AuthenticationError)
        }
    }
}

impl <R, RP, S> TodoUseCaseInstractor<R, RP, S>
{
    pub fn new(travel_repository: R, todo_repository: RP, todo_id_service: S) -> Self {
        Self {
            travel_repository,
            todo_repository,
            todo_id_service
        }
    }
}

impl<R, RP, S> ToDoUseCases for TodoUseCaseInstractor<R, RP, S>
    where R: TravelRepository, RP: TodoRepository, S: TodoIdService
{
    async fn travel_to_do_list_group(&self, travel_id: &str) -> Result<Vec<ToDoListGroupDto>, TravelError> {

        let travel_id_struct = TravelId::try_from(travel_id)?;

        match self.todo_repository.list_todo_list_group(&travel_id_struct).await {
            Ok(todo_list_group) => Ok(todo_list_group.iter().map(|el| ToDoListGroupDto::from(el)).collect()),
            Err(e) => Err(e)
        }
    }

    async fn get_todo_list_group(&self, travel_id: &str, to_do_list_group_id: &u32) -> Result<Option<ToDoListGroupDto>, TravelError> {
        let travel_id_struct = TravelId::try_from(travel_id)?;

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
        let travel_id_struct = TravelId::try_from(travel_id)?;
        let todo_list_group_id = TodoListGroupId::from(todo_list_group_id);
        let todo_id = TodoId::from(todo_id);

        match self.todo_repository.find_todo_by_id(&travel_id_struct, &todo_list_group_id, &todo_id).await {
            Ok(todo) => {
                match todo {
                    Some(t) => Ok(Some(ToDoDto::from(&t))),
                    None => Ok(None)
                }
            },
            Err(e) => Err(e)
        }

    }

    async fn create_new_todo_list_group(&self, user_id: &str, travel_id: &str, name: &str, tz: Option<i64>) -> Result<ToDoListGroupDto, TravelError> {
        // check authentication
        self.check_authentication(user_id, travel_id).await?;

        let travel_id = TravelId::try_from(travel_id)?;

        let latest_todo_list_group_id = self.todo_id_service.get_todo_list_group_id(&travel_id).await?;
        let todo_list_group_id = TodoListGroupId::from(&latest_todo_list_group_id);

        let latest_todo_id = self.todo_id_service.get_todo_id(&travel_id, &todo_list_group_id).await?;
        let todo_id = TodoId::from(&latest_todo_id);

        let todo = Todo::new(&todo_id, "summary", None, tz, None)?;

        let todo_list_group = TodoListGroup::new(&travel_id, &todo_list_group_id, name, vec![todo], None)?;

        self.todo_repository.save_todo_list_group(&todo_list_group).await?;

        Ok(ToDoListGroupDto::from(&todo_list_group))
    }

    async fn crate_new_todo(&self, user_id: &str, travel_id: &str, todo_list_group_id: &u32, summary: &str, description: Option<&str>, due_date: Option<&str>) -> Result<ToDoDto, TravelError> {
        // check authentication
        self.check_authentication(user_id, travel_id).await?;

        let travel_id = TravelId::try_from(travel_id)?;
        let todo_list_group_id = TodoListGroupId::from(todo_list_group_id);

        let latest_todo_id = self.todo_id_service.get_todo_id(&travel_id, &todo_list_group_id).await?;
        let todo_id = TodoId::from(&latest_todo_id);

        let due_date = match due_date {
            None => None,
            Some(d) => {
                match chrono::DateTime::parse_from_rfc3339(d) {
                    Ok(due) => {
                        let timestamp = due.timestamp();
                        Some(timestamp)
                    },
                    Err(_) => return Err(TravelError::UseCaseError("Invalid Datetime is provided".to_string()))
                }
            }
        };

        let todo = Todo::new(&todo_id, summary, description, due_date, None)?;
        
        self.todo_repository.save_todo(&travel_id, &todo_list_group_id, &todo).await?;

        Ok(ToDoDto::from(&todo))
    }

    async fn update_todo_list_group(&self, user_id: &str, travel_id: &str, todo_list_group_id: &u32, name: &str, tz: Option<i32>) -> Result<ToDoListGroupDto, TravelError> {
        // check authentication
        self.check_authentication(user_id, travel_id).await?;
        
        let travel_id = TravelId::try_from(travel_id)?;
        let todo_list_group_id = TodoListGroupId::from(todo_list_group_id);
        
        let todo_list_group = match self.todo_repository.find_todo_list_group_by_id(&travel_id, &todo_list_group_id).await? {
            Some(t) => t,
            None => return Err(TravelError::NotFound("The requested todo list group is not found.".to_string()))
        };
        
        let updated = todo_list_group.update(name, tz)?;
        
        self.todo_repository.save_todo_list_group(&updated).await?;
        
        Ok(ToDoListGroupDto::from(&updated))
    }

    async fn update_todo(&self, user_id: &str, travel_id: &str, todo_list_group_id: &u32, todo_id: &u32, summary: &str, description: Option<&str>, due_date: Option<&str>) -> Result<ToDoDto, TravelError> {
        // check authentication
        self.check_authentication(user_id, travel_id).await?;
        
        let travel_id = TravelId::try_from(travel_id)?;
        let todo_list_group_id = TodoListGroupId::from(todo_list_group_id);
        let todo_id = TodoId::from(todo_id);
        
        let todo = match self.todo_repository.find_todo_by_id(&travel_id, &todo_list_group_id, &todo_id).await? {
            Some(t) => t,
            None => return Err(TravelError::NotFound("The requested todo is not found".to_string()))
        };
        
        let updated_todo = todo.update(summary, description)?;
        
        self.todo_repository.save_todo(&travel_id, &todo_list_group_id, &updated_todo).await?;
        
        Ok(ToDoDto::from(&updated_todo))
    }

    async fn toggle_todo_done(&self, user_id: &str, travel_id: &str, todo_list_group_id: &u32, todo_id: &u32) -> Result<ToDoDto, TravelError> {
        // check authentication
        self.check_authentication(user_id, travel_id).await?;

        let travel_id = TravelId::try_from(travel_id)?;
        let todo_list_group_id = TodoListGroupId::from(todo_list_group_id);
        let todo_id = TodoId::from(todo_id);

        let todo = match self.todo_repository.find_todo_by_id(&travel_id, &todo_list_group_id, &todo_id).await? {
            Some(t) => t,
            None => return Err(TravelError::NotFound("The requested todo is not found".to_string()))
        };

        let updated_todo = todo.toggle_todo();

        self.todo_repository.save_todo(&travel_id, &todo_list_group_id, &updated_todo).await?;

        Ok(ToDoDto::from(&updated_todo))
    }

}
