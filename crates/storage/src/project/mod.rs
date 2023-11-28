use crate::{Storage, StorageResult};
use dto::*;
use rc_entity::{prelude::ProjectDb, sea_orm::TransactionTrait};

mod dto;

pub struct ProjectStorage<'a> {
    storage: &'a Storage,
}

impl<'a> ProjectStorage<'a> {
    pub async fn create_project(
        &self,
        form: ProjectStorageForm,
    ) -> StorageResult<ProjectStorageModel> {
        let option = form.into_option();

        let begin = self.storage.conn.begin().await?;

        let db = ProjectDb::new(&begin);

        let model = db.create(option).await?.into();

        begin.commit().await?;

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
