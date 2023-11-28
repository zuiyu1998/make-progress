use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceKind {}

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error(transparent)]
    Kind(#[from] ServiceKind),
}

pub type ServiceResult<T, E = ServiceError> = std::result::Result<T, E>;
