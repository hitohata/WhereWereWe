use thiserror::Error;

#[derive(Error, Debug)]
pub enum TravelError {
    #[error("{0}")]
    DomainError(String),
    #[error("{0}")]
    DBError(String),
    #[error("{0} not found")]
    NotFound(String),
    #[error("Authentication Error")]
    AuthenticationError
}
