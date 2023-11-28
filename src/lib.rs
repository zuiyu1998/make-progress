mod error;
mod service;

pub use error::*;
pub use service::Service;

pub mod prelude {
    pub use crate::service::*;
}
