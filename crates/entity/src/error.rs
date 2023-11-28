use thiserror::Error;

#[derive(Debug, Error)]
pub enum EntityKind {}

#[derive(Debug, Error)]
pub enum EntityError {
    #[error(transparent)]
    Kind(#[from] EntityKind),
}

pub type EntityResult<T, E = EntityError> = std::result::Result<T, E>;
