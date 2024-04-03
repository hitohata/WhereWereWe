use thiserror::Error;

#[derive(Error, Debug)]
pub (crate) enum UsersError {
    #[error("[UsernameError]: {0}")]
    UsernameError(String),
    #[error("[UserIdError]: Invalid UUID")]
    InvalidUUID,
    #[error("[Use Case Error]: User is not find")]
    UserNotFind
}