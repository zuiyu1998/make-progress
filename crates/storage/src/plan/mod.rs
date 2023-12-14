use crate::StorageResult;
use rc_entity::{prelude::PlanDb, sea_orm::ConnectionTrait};

mod dto;

pub use dto::*;

pub struct PlanStorage<'a, C> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> PlanStorage<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        PlanStorage { conn }
    }

    pub async fn delete(&self, id: i32) -> StorageResult<()> {
        let db = PlanDb::new(self.conn);

        db.delete(id).await?;

        Ok(())
    }

    pub async fn create_plan(&self, form: PlanStorageForm) -> StorageResult<PlanStorageModel> {
        let option = form.into_option();

        let db = PlanDb::new(self.conn);

        let model = db.create(option).await?.into();

        Ok(model)
    }

    pub async fn find_plan(&self, id: i32) -> StorageResult<PlanStorageModel> {
        let db = PlanDb::new(self.conn);

        let model = db.get(id).await?.into();

        Ok(model)
    }

    pub fn update_plan(&self, _update: PlanStorageUpdate) -> StorageResult<PlanStorageModel> {
        todo!()
    }

    pub async fn list(&self, params: PlanStorageListParams) -> StorageResult<PlanStorageList> {
        let page_size = params.page_size;
        let mut page = params.page;

        let db = PlanDb::new(self.conn);

        let list = db.list(params.into()).await?;

        let mut has_next = true;

        if list.data.len() < page_size as usize {
            has_next = false;
        }

        if has_next {
            page = page + 1;
        }

        Ok(PlanStorageList {
            total: list.total,
            data: list
                .data
                .into_iter()
                .map(|model| model.into())
                .collect::<Vec<PlanStorageModel>>(),
            page,
            page_size,
            has_next,
        })
    }
}
