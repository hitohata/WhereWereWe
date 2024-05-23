pub mod user_interface {
    use utils::infrastructure::controller::responses::{ErrHttpResponse, ErrorStatus, OkHttpResponse, OkStatus};
    use crate::dtos::user_dto::UserDto;
    use crate::use_case::user_use_case_implementation::UserUseCasesInteractor;
    use crate::use_case::user_use_cases::UserUseCases;
    use crate::repository::user_repository::UserRepositoryConcrete;
    use crate::errors::errors::UsersError;
    
    struct UserInterface {
        use_case: UserUseCasesInteractor<UserRepositoryConcrete>
    }

    impl UserInterface {
        /// create interface
        pub async fn new() -> Self {
            
            let repository = UserRepositoryConcrete::default().await;
            let use_case = UserUseCasesInteractor::new(repository);
            
            Self {
                use_case
            }
        }
        
        async fn add_partner(&self, user_id: &str, partner_id: &str) -> Result<OkHttpResponse<UserDto>, ErrHttpResponse>{

            let result = self.use_case.add_partner(user_id, partner_id).await;
            
            let user= match result {
                Ok(u) => u,
                Err(UsersError::UserNotFind(e)) => {
                    return Err(ErrHttpResponse::new(ErrorStatus::BadRequest, Some(&e)))
                },
                _ => return Err(ErrHttpResponse::new(ErrorStatus::InternalServerError, Some("unexpected error")))
            };
            
            Ok(OkHttpResponse::new(OkStatus::Accepted, user))
        }

        async fn remove_partner(&self, user_id: &str, partner_id: &str) -> Result<OkHttpResponse<UserDto>, ErrHttpResponse>{

            let result = self.use_case.remove_partner(user_id, partner_id).await;

            let user= match result {
                Ok(u) => u,
                Err(UsersError::UserNotFind(e)) => {
                    return Err(ErrHttpResponse::new(ErrorStatus::BadRequest, Some(&e)))
                },
                _ => return Err(ErrHttpResponse::new(ErrorStatus::InternalServerError, Some("unexpected error")))
            };

            Ok(OkHttpResponse::new(OkStatus::Accepted, user))
        }
        
        async fn change_name(&self, user_id: &str, name: &str) -> Result<OkHttpResponse<UserDto>, ErrHttpResponse>{

            let result = self.use_case.change_name(user_id, name).await;

            let user= match result {
                Ok(u) => u,
                Err(UsersError::UserNotFind(e)) => {
                    return Err(ErrHttpResponse::new(ErrorStatus::BadRequest, Some(&e)))
                },
                _ => return Err(ErrHttpResponse::new(ErrorStatus::InternalServerError, Some("unexpected error")))
            };

            Ok(OkHttpResponse::new(OkStatus::Accepted, user))
        }
    }
}

