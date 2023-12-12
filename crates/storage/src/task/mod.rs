use crate::StorageResult;
use rc_entity::{
    prelude::{TaskColumn, TaskDb, TaskEntity, TaskModelDto},
    sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter},
};

mod dto;

pub use dto::*;

pub struct TaskStorage<'a, C> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> TaskStorage<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        TaskStorage { conn }
    }

    pub async fn delete(&self, id: i32) -> StorageResult<()> {
        let db = TaskDb::new(self.conn);

        db.delete(id).await?;

        Ok(())
    }

    pub async fn create_task(&self, form: TaskStorageForm) -> StorageResult<TaskStorageModel> {
        let option = form.into_option();

        let db = TaskDb::new(self.conn);

        let model = db.create(option).await?.into();

        Ok(model)
    }

    pub async fn find_task(&self, id: i32) -> StorageResult<TaskStorageModel> {
        let db = TaskDb::new(self.conn);

        let model = db.get(id).await?.into();

        Ok(model)
    }

    pub fn update_task(&self, _update: TaskStorageUpdate) -> StorageResult<TaskStorageModel> {
        todo!()
    }

    pub async fn list(&self, params: TaskStorageListParams) -> StorageResult<TaskStorageList> {
        let mut sql = TaskEntity::find();

        if let Some(project_id) = params.project_id {
            sql = sql.filter(TaskColumn::ProjectId.eq(project_id));
        }

        let total = sql.clone().count(self.conn).await?;
        let paginator = sql.paginate(self.conn, params.page_size);
        let data = paginator
            .fetch_page(params.page)
            .await?
            .into_iter()
            .map(|item| TaskModelDto::new(item).into())
            .collect::<Vec<TaskStorageModel>>();

        let mut has_next = true;

        if data.len() < params.page_size as usize {
            has_next = false;
        }

        Ok(TaskStorageList {
            total,
            data,
            page: params.page,
            page_size: params.page_size,
            has_next,
        })
    }
}
