use thiserror::Error;

#[derive(Error, Debug)]
pub (crate) enum UsersError {
    #[error("[UsernameError]: {0}")]
    UsernameError(String)
}