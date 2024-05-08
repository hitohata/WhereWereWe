use thiserror::Error;

#[derive(Error, Debug)]
pub enum TravelError {
    #[error("{0}")]
    DomainError(String)
}
