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

    pub async fn delete(&self, id: i32) -> StorageResult<()> {
        let db = ProjectDb::new(self.conn);

        db.delete(id).await?;

        Ok(())
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

    pub async fn find_project(&self, id: i32) -> StorageResult<ProjectStorageModel> {
        let db = ProjectDb::new(self.conn);

        let model = db.get(id).await?.into();

        Ok(model)
    }

    pub fn update_project(
        &self,
        _update: ProjectStorageUpdate,
    ) -> StorageResult<ProjectStorageModel> {
        todo!()
    }

    pub async fn list(
        &self,
        params: ProjectStorageListParams,
    ) -> StorageResult<ProjectStorageList> {
        let page_size = params.page_size;
        let mut page = params.page;

        let db = ProjectDb::new(self.conn);

        let list = db.list(params.into()).await?;

        let mut has_next = true;

        if list.data.len() < page_size as usize {
            has_next = false;
        }

        if has_next {
            page = page + 1;
        }

        Ok(ProjectStorageList {
            total: list.total,
            data: list
                .data
                .into_iter()
                .map(|model| model.into())
                .collect::<Vec<ProjectStorageModel>>(),
            page,
            page_size,
            has_next,
        })
    }
}
