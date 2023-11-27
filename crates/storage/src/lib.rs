mod project;

mod error;

pub struct Storage;

pub use error::*;

pub mod prelude {
    pub use crate::project::ProjectStorage;
}
