use make_progress_core::ServiceError;
use thiserror::Error;

use serde::{Deserialize, Serialize};

#[derive(Debug, Error, Deserialize, Serialize)]
pub enum TauriKind {}

#[derive(Debug, Error, Deserialize, Serialize)]
pub enum TauriError {
    #[error(transparent)]
    Kind(#[from] TauriKind),
    #[error("ServiceError:{0}")]
    ServiceError(String),
}

impl From<ServiceError> for TauriError {
    fn from(value: ServiceError) -> Self {
        Self::ServiceError(value.to_string())
    }
}

pub type TauriResult<T, E = TauriError> = std::result::Result<T, E>;
