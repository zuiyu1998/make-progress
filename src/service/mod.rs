use rc_storage::Storage;

mod project;

#[derive(Clone)]
pub struct Service {
    pub storage: Storage,
}

impl Service {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
