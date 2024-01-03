use crate::StorageResult;
use rc_entity::sea_orm::{ActiveModelTrait, ConnectionTrait};

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
        Ok(())
    }

    pub async fn create_plan(&self, form: PlanForm) -> StorageResult<Plan> {
        let active = form.get_active_model();

        let model = active.insert(self.conn).await?;

        Ok(Plan::from(model))
    }

    pub async fn find_plan(&self, id: i32) -> StorageResult<()> {
        todo!()
    }

    pub fn update_plan(&self) -> StorageResult<()> {
        todo!()
    }

    pub async fn list(&self) -> StorageResult<()> {
        todo!()
    }
}
