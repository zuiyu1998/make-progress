use rc_storage::sea_orm::DatabaseConnection;
mod plan;
mod project;
mod task;

pub use plan::*;
pub use project::*;
pub use task::*;

#[derive(Clone)]
pub struct Service {
    pub conn: DatabaseConnection,
}

impl Service {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}
