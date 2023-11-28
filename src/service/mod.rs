use rc_storage::Storage;

mod project;

#[derive(Clone)]
pub struct Service {
    pub storage: Storage,
}
