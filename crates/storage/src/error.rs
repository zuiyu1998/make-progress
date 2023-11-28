use rc_entity::{sea_orm::DbErr, EntityError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageKind {}

#[derive(Debug, Error)]
pub enum StorageError {
    #[error(transparent)]
    Kind(#[from] StorageKind),
    #[error(transparent)]
    EntityError(#[from] EntityError),
    #[error(transparent)]
    DbErr(#[from] DbErr),
}

pub type StorageResult<T, E = StorageError> = std::result::Result<T, E>;
