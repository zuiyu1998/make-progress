use crate::StorageResult;
use rc_entity::{
    prelude::{PlanDb, PlanEntity, PlanModelDto},
    sea_orm::{ConnectionTrait, EntityTrait, PaginatorTrait},
};

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
        let total = PlanEntity::find().count(self.conn).await?;
        let sql = PlanEntity::find().paginate(self.conn, params.page_size);
        let data = sql
            .fetch_page(params.page)
            .await?
            .into_iter()
            .map(|item| PlanModelDto::new(item).into())
            .collect::<Vec<PlanStorageModel>>();

        let mut has_next = true;

        if data.len() < params.page_size as usize {
            has_next = false;
        }

        Ok(PlanStorageList {
            total,
            data,
            page: params.page,
            page_size: params.page_size,
            has_next,
        })
    }
}