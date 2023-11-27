use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageKind {}

#[derive(Debug, Error)]
pub enum StorageError {
    #[error(transparent)]
    Kind(#[from] StorageKind),
}

pub type StorageResult<T, E = StorageError> = std::result::Result<T, E>;
