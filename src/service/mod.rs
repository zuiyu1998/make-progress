use rc_storage::Storage;

mod plan;
mod project;

pub use project::*;

#[derive(Clone)]
pub struct Service {
    pub storage: Storage,
}

impl Service {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
