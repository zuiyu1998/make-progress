mod project;

pub mod error;
pub use sea_orm;

pub use error::*;

pub mod prelude {

    pub use crate::project::*;
}
