use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum TravelError {
    #[error("{0}")]
    DomainError(String)
}
