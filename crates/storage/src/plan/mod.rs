use crate::StorageResult;
use rc_entity::{
    prelude::PlanEntity,
    sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait, PaginatorTrait},
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
        Ok(())
    }

    pub async fn create_plan(&self, form: PlanForm) -> StorageResult<Plan> {
        let active = form.get_active_model();

        let model = active.insert(self.conn).await?;

        Ok(Plan::from(model))
    }

    pub async fn find_plan(&self, id: i32) -> StorageResult<Option<Plan>> {
        let model = PlanEntity::find_by_id(id)
            .one(self.conn)
            .await?
            .and_then(|item| Some(Plan::from(item)));

        Ok(model)
    }

    pub fn update_plan(&self) -> StorageResult<()> {
        todo!()
    }

    pub async fn list(&self, params: PlanParams) -> StorageResult<PlanList> {
        let sql = PlanEntity::find();

        let paginator = sql.paginate(self.conn, params.page_size as u64);

        let list = paginator.fetch_page(params.page as u64).await?;

        let mut has_next = true;

        if list.len() < params.page_size as usize {
            has_next = false;
        }

        let plan_list = PlanList {
            data: list
                .into_iter()
                .map(|item| Plan::from(item))
                .collect::<Vec<Plan>>(),
            has_next,
            page: params.page,
            page_size: params.page_size,
        };

        Ok(plan_list)
    }
}
