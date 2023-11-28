mod project;

pub mod error;

pub mod prelude {

    pub use crate::project::{
        Column as ProjectColumn, Entity as ProjectEntity, Model as ProjectModel,
    };
}
