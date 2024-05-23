use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("{0}")]
    ServiceError(String),
    #[error("{0}")]
    DbError(String)
}