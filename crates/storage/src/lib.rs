use rc_entity::sea_orm::DatabaseConnection;

mod plan;
mod project;

mod error;

#[derive(Clone)]
pub struct Storage {
    pub conn: DatabaseConnection,
}

impl Storage {
    pub fn new(conn: DatabaseConnection) -> Self {
        Storage { conn }
    }
}

pub use error::*;

pub mod prelude {
    pub use crate::plan::*;
    pub use crate::project::*;
}

pub use chrono;
pub use rc_entity::sea_orm;
