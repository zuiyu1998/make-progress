use crate::{Storage, StorageResult};

pub struct ProjectStorageForm {}
pub struct ProjectStorageUpdate {}

pub struct ProjectStorageModel {}

pub struct ProjectStorageListParams {}

pub struct ProjectStorage<'a> {
    storage: &'a Storage,
}

impl<'a> ProjectStorage<'a> {
    pub fn create_project(&self, _form: ProjectStorageForm) -> StorageResult<ProjectStorageModel> {
        todo!()
    }

    pub fn find_project(&self, _id: i32) -> StorageResult<Option<ProjectStorageModel>> {
        Ok(None)
    }

    pub fn update_project(
        &self,
        _update: ProjectStorageUpdate,
    ) -> StorageResult<ProjectStorageModel> {
        todo!()
    }

    pub fn list(
        &self,
        _params: ProjectStorageListParams,
    ) -> StorageResult<Vec<ProjectStorageModel>> {
        todo!()
    }
}
