pub mod travel_interface {
    use std::fs::metadata;
    use crate::repository::{
        travel_repository::TravelRepositoryConcrete,
        todo_repository::TodoRepositoryConcrete
    };
    use crate::service::todo_id_service::TodoIdServiceConcrete;
    use crate::use_cases::todo_use_cases::{TodoUseCaseInstractor, ToDoUseCases};
    use crate::use_cases::travel_use_case::{TravelUseCases, TravelUseCasesInteractor};
    use tokio;
    use utils::infrastructure::controller::responses::{ErrHttpResponse, ErrorStatus, OkHttpResponse, OkStatus};
    use crate::dtos::todo::{ToDoDto, ToDoListGroupDto};
    use crate::dtos::travel::TravelDto;
    use crate::errors::errors::TravelError;

    pub struct TravelInterface {
        travel_use_case: TravelUseCasesInteractor<TravelRepositoryConcrete>,
        todo_use_case: TodoUseCaseInstractor<TravelRepositoryConcrete, TodoRepositoryConcrete, TodoIdServiceConcrete>,
    }

    impl TravelInterface {
        pub async fn new() -> Self {

            let (travel_repository, todo_repository, todo_id_service) = tokio::join!(
                TravelRepositoryConcrete::default(),
                TodoRepositoryConcrete::default(),
                TodoIdServiceConcrete::default(),
            );

            Self {
                travel_use_case: TravelUseCasesInteractor::new(travel_repository.clone()),
                todo_use_case: TodoUseCaseInstractor::new(travel_repository, todo_repository, todo_id_service),
            }
        }

        /// get travel by id
        pub async fn get_travel(&self, travel_id: &str, user_id: &str) -> Result<OkHttpResponse<Option<TravelDto>>, ErrHttpResponse> {
            let response = self.travel_use_case.get_travel(travel_id, user_id).await;

            match response {
                Ok(travel) => Ok(OkHttpResponse::new(OkStatus::OK, travel)),
                Err(TravelError::DomainError(e)) => Err(ErrHttpResponse::new(ErrorStatus::BadRequest, Some(e.as_str()))),
                Err(TravelError::UseCaseError(e)) => Err(ErrHttpResponse::new(ErrorStatus::BadRequest, Some(e.as_str()))),
                Err(TravelError::AuthenticationError) => Err(ErrHttpResponse::new(ErrorStatus::Forbidden, Some("You cannot access to this travel"))),
                _ => Err(ErrHttpResponse::new(ErrorStatus::InternalServerError, None)),
            }
        }

        /// create new travel
        pub async fn crate_new_travel(&self, user_id: &str, travel_name: &str, start_date: &str, end_date: Option<&str>) -> Result<OkHttpResponse<TravelDto>, ErrHttpResponse> {
            let response = self.travel_use_case.create_new_travel(user_id, travel_name, start_date, end_date).await;

            match response {
                Ok(travel) => Ok(OkHttpResponse::new(OkStatus::Created, travel)),
                Err(TravelError::DomainError(e)) => Err(ErrHttpResponse::new(ErrorStatus::BadRequest, Some(e.as_str()))),
                Err(TravelError::UseCaseError(e)) => Err(ErrHttpResponse::new(ErrorStatus::BadRequest, Some(e.as_str()))),
                Err(_) => Err(ErrHttpResponse::new(ErrorStatus::InternalServerError, None))
            }
        }

        pub async fn modify_travel(&self, travel_id: &str, travel_name: &str, start_date: &str, end_date: Option<&str>, travelers: &Vec<&str>, involved_users: &Vec<&str>, user_id: &str) -> Result<OkHttpResponse<TravelDto>, ErrHttpResponse> {
            let response =  self.travel_use_case.modify_travel(travel_id, travel_name, start_date, end_date, travelers, involved_users, user_id).await;

            match response {
                Ok(travel) => Ok(OkHttpResponse::new(OkStatus::OK, travel)),
                Err(TravelError::DomainError(e)) => Err(ErrHttpResponse::new(ErrorStatus::BadRequest, Some(e.as_str()))),
                Err(TravelError::UseCaseError(e)) => Err(ErrHttpResponse::new(ErrorStatus::BadRequest, Some(e.as_str()))),
                Err(TravelError::AuthenticationError) => Err(ErrHttpResponse::new(ErrorStatus::Forbidden, Some("You cannot access to this travel"))),
                _ => Err(ErrHttpResponse::new(ErrorStatus::InternalServerError, None))
            }
        }

        pub async fn todo_list(&self, travel_id: &str) -> Result<OkHttpResponse<Vec<ToDoListGroupDto>>, ErrHttpResponse> {
            let response = self.todo_use_case.travel_to_do_list_group(travel_id).await;

            match response {
                Ok(todos) => Ok(OkHttpResponse::new(OkStatus::OK, todos)),
                Err(TravelError::DomainError(e)) => Err(ErrHttpResponse::new(ErrorStatus::BadRequest, Some(e.as_str()))),
                Err(TravelError::UseCaseError(e)) => Err(ErrHttpResponse::new(ErrorStatus::BadRequest, Some(e.as_str()))),
                Err(TravelError::AuthenticationError) => Err(ErrHttpResponse::new(ErrorStatus::Forbidden, Some("You cannot access to this todo list"))),
                Err(_) => Err(ErrHttpResponse::new(ErrorStatus::InternalServerError, None))
            }
        }
        
        pub async fn todo_list_group(&self, travel_id: &str, todo_list_group_id: &u32) -> Result<OkHttpResponse<Option<ToDoListGroupDto>>, ErrHttpResponse> {
            let result = self.todo_use_case.get_todo_list_group(travel_id, todo_list_group_id).await;
            
            match result {
                Ok(todos) => Ok(OkHttpResponse::new(OkStatus::OK, todos)),
                Err(TravelError::DomainError(e)) => Err(ErrHttpResponse::new(ErrorStatus::BadRequest, Some(e.as_str()))),
                Err(TravelError::UseCaseError(e)) => Err(ErrHttpResponse::new(ErrorStatus::BadRequest, Some(e.as_str()))),
                Err(TravelError::AuthenticationError) => Err(ErrHttpResponse::new(ErrorStatus::Forbidden, Some("You cannot access to this todo list"))),
                Err(_) => Err(ErrHttpResponse::new(ErrorStatus::InternalServerError, None))
            }
        }
        
        pub async fn todo(&self, travel_id: &str, todo_list_group_id: &u32, todo_id: &u32) -> Result<OkHttpResponse<Option<ToDoDto>>, ErrHttpResponse> {
            let result = self.todo_use_case.get_todo(travel_id, todo_list_group_id, todo_id).await;

            match result {
                Ok(todo) => Ok(OkHttpResponse::new(OkStatus::OK, todo)),
                Err(TravelError::DomainError(e)) => Err(ErrHttpResponse::new(ErrorStatus::BadRequest, Some(e.as_str()))),
                Err(TravelError::UseCaseError(e)) => Err(ErrHttpResponse::new(ErrorStatus::BadRequest, Some(e.as_str()))),
                Err(TravelError::AuthenticationError) => Err(ErrHttpResponse::new(ErrorStatus::Forbidden, Some("You cannot access to this todo"))),
                Err(_) => Err(ErrHttpResponse::new(ErrorStatus::InternalServerError, None))
            }
        }
        
        pub async fn create_new_todo_list_group(&self, user_id: &str, travel_id: &str, name: &str, tz: Option<i64>) -> Result<OkHttpResponse<ToDoListGroupDto>, ErrHttpResponse> {
            let result = self.todo_use_case.create_new_todo_list_group(user_id, travel_id, name, tz).await;

            match result {
                Ok(todo) => Ok(OkHttpResponse::new(OkStatus::OK, todo)),
                Err(TravelError::DomainError(e)) => Err(ErrHttpResponse::new(ErrorStatus::BadRequest, Some(e.as_str()))),
                Err(TravelError::AuthenticationError) => Err(ErrHttpResponse::new(ErrorStatus::Forbidden, Some("You cannot access to this todo"))),
                Err(_) => Err(ErrHttpResponse::new(ErrorStatus::InternalServerError, None))
            }
        }
        
        pub async fn create_todo(
            &self,
            user_id: &str,
            travel_id: &str,
            todo_list_group_id: &u32,
            name: &str,
            description: Option<&str>,
            due_date: Option<&str>,
        ) -> Result<OkHttpResponse<ToDoDto>, ErrHttpResponse> {
            let result = self.todo_use_case.create_new_todo(
                user_id,
                travel_id,
                todo_list_group_id,
                name,
                description,
                due_date,
            ).await;

            match result {
                Ok(todo) => Ok(OkHttpResponse::new(OkStatus::OK, todo)),
                Err(TravelError::DomainError(e)) => Err(ErrHttpResponse::new(ErrorStatus::BadRequest, Some(e.as_str()))),
                Err(TravelError::UseCaseError(e)) => Err(ErrHttpResponse::new(ErrorStatus::BadRequest, Some(e.as_str()))),
                Err(TravelError::AuthenticationError) => Err(ErrHttpResponse::new(ErrorStatus::Forbidden, Some("You cannot access to this todo"))),
                Err(_) => Err(ErrHttpResponse::new(ErrorStatus::InternalServerError, None))
            }
        }
        
        pub async fn update_todo_list_group(
            &self,
            user_id: &str,
            travel_id: &str,
            todo_list_group_id: &u32,
            name: &str,
            tz: Option<i32>,
        ) -> Result<OkHttpResponse<ToDoListGroupDto>, ErrHttpResponse> {
            let result = self.todo_use_case.update_todo_list_group(
                user_id,
                travel_id,
                todo_list_group_id,
                name,
                tz,
            ).await;

            match result {
                Ok(todo) => Ok(OkHttpResponse::new(OkStatus::OK, todo)),
                Err(TravelError::DomainError(e)) => Err(ErrHttpResponse::new(ErrorStatus::BadRequest, Some(e.as_str()))),
                Err(TravelError::UseCaseError(e)) => Err(ErrHttpResponse::new(ErrorStatus::BadRequest, Some(e.as_str()))),
                Err(TravelError::AuthenticationError) => Err(ErrHttpResponse::new(ErrorStatus::Forbidden, Some("You cannot access to this todo"))),
                Err(_) => Err(ErrHttpResponse::new(ErrorStatus::InternalServerError, None))
            }
        }
        
        pub async fn update_todo(
            &self,
            user_id: &str,
            travel_id: &str,
            todo_list_group_id: &u32,
            todo_id: &u32,
            summary: &str,
            description: Option<&str>,
            due_date: Option<&str>,
        ) -> Result<OkHttpResponse<ToDoDto>, ErrHttpResponse> {
            let result = self.todo_use_case.update_todo(
                user_id,
                travel_id,
                todo_list_group_id,
                todo_id,
                summary,
                description,
                due_date
            ).await;

            match result {
                Ok(todo) => Ok(OkHttpResponse::new(OkStatus::OK, todo)),
                Err(TravelError::DomainError(e)) => Err(ErrHttpResponse::new(ErrorStatus::BadRequest, Some(e.as_str()))),
                Err(TravelError::UseCaseError(e)) => Err(ErrHttpResponse::new(ErrorStatus::BadRequest, Some(e.as_str()))),
                Err(TravelError::AuthenticationError) => Err(ErrHttpResponse::new(ErrorStatus::Forbidden, Some("You cannot access to this todo"))),
                Err(_) => Err(ErrHttpResponse::new(ErrorStatus::InternalServerError, None))
            }
        }
        
        pub async fn toggle_todo(&self, user_id: &str, travel_id: &str, todo_list_group_id: &u32, todo_id: &u32) -> Result<OkHttpResponse<ToDoDto>, ErrHttpResponse> {
            let result = self.todo_use_case.toggle_todo_done(
                user_id,
                travel_id,
                todo_list_group_id,
                todo_id
            ).await;

            match result {
                Ok(todo) => Ok(OkHttpResponse::new(OkStatus::OK, todo)),
                Err(TravelError::DomainError(e)) => Err(ErrHttpResponse::new(ErrorStatus::BadRequest, Some(e.as_str()))),
                Err(TravelError::UseCaseError(e)) => Err(ErrHttpResponse::new(ErrorStatus::BadRequest, Some(e.as_str()))),
                Err(TravelError::NotFound(e)) => Err(ErrHttpResponse::new(ErrorStatus::BadRequest, Some(e.as_str()))),
                Err(TravelError::AuthenticationError) => Err(ErrHttpResponse::new(ErrorStatus::Forbidden, Some("You cannot access to this todo"))),
                Err(_) => Err(ErrHttpResponse::new(ErrorStatus::InternalServerError, None))
            }
        }
    }
}