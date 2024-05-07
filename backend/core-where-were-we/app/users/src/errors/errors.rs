use thiserror::Error;

#[derive(Error, Debug)]
pub enum UsersError {
    #[error("{0}")]
    UsernameError(String),
    #[error("Invalid UUID")]
    InvalidUUID,
    #[error("User is not find: {0}")]
    UserNotFind(String),
    #[error("Dynamo Connection Error")]
    Connection(#[from] aws_sdk_dynamodb::Error),
    #[error("DB Error: {0}")]
    DBError(String),
    #[error("Use Case Error: {0}")]
    UsersUseCaseError(String)
}