use crate::StorageResult;
use rc_entity::{prelude::ProjectDb, sea_orm::ConnectionTrait};

mod dto;

pub use dto::*;

pub struct ProjectStorage<'a, C> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> ProjectStorage<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        ProjectStorage { conn }
    }

    pub async fn create_project(
        &self,
        form: ProjectStorageForm,
    ) -> StorageResult<ProjectStorageModel> {
        let option = form.into_option();

        let db = ProjectDb::new(self.conn);

        let model = db.create(option).await?.into();

        Ok(model)
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
