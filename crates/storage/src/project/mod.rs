use crate::StorageResult;
use rc_entity::{
    prelude::{ProjectColumn, ProjectDb, ProjectEntity, ProjectModelDto},
    sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter},
};

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
        let total = ProjectEntity::find().count(self.conn).await?;
        let sql = ProjectEntity::find().paginate(self.conn, params.page_size);
        let data = sql
            .fetch_page(params.page)
            .await?
            .into_iter()
            .map(|item| ProjectModelDto::new(item).into())
            .collect::<Vec<ProjectStorageModel>>();

        let mut has_next = true;

        if data.len() < params.page_size as usize {
            has_next = false;
        }

        Ok(ProjectStorageList {
            total,
            data,
            page: params.page,
            page_size: params.page_size,
            has_next,
        })
    }
}
