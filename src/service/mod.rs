use rc_storage::Storage;

mod plan;
mod project;
mod task;

pub use plan::*;
pub use project::*;
pub use task::*;

#[derive(Clone)]
pub struct Service {
    pub storage: Storage,
}

impl Service {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
