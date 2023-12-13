use crate::StorageResult;
use rc_entity::{prelude::TaskDb, sea_orm::ConnectionTrait};

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
        let page_size = params.page_size;
        let mut page = params.page;

        let db = TaskDb::new(self.conn);

        let list = db.list(params.into()).await?;

        let mut has_next = true;

        if list.data.len() < page_size as usize {
            has_next = false;
        }

        if has_next {
            page = page + 1;
        }

        Ok(TaskStorageList {
            total: list.total,
            data: list
                .data
                .into_iter()
                .map(|model| model.into())
                .collect::<Vec<TaskStorageModel>>(),
            page,
            page_size,
            has_next,
        })
    }
}
