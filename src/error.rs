use rc_storage::{sea_orm::DbErr, StorageError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceKind {}

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error(transparent)]
    Kind(#[from] ServiceKind),
    #[error(transparent)]
    DbErr(#[from] DbErr),
    #[error(transparent)]
    StorageError(#[from] StorageError),
}

pub type ServiceResult<T, E = ServiceError> = std::result::Result<T, E>;
