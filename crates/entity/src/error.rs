use sea_orm::DbErr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EntityError {
    #[error(transparent)]
    DbErr(#[from] DbErr),
}

pub type EntityResult<T, E = EntityError> = std::result::Result<T, E>;
