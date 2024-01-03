use crate::{Service, ServiceResult};
use migration::sea_orm::TransactionTrait;
use rc_storage::prelude::PlanStorage;

pub use rc_storage::prelude::{Plan, PlanForm};

pub struct PlanService<'a> {
    service: &'a Service,
}

impl<'a> PlanService<'a> {
    pub fn new(service: &'a Service) -> Self {
        Self { service }
    }

    //创建计划
    pub async fn create_plan(&self, form: PlanForm) -> ServiceResult<Plan> {
        let begin = self.service.conn.begin().await?;

        let storage = PlanStorage::new(&begin);

        let plan = storage.create_plan(form).await?;

        begin.commit().await?;

        Ok(plan)
    }

    pub async fn get_plan(&self, plan_id: i32) -> ServiceResult<()> {
        todo!()
    }

    ///获取项目列表
    pub async fn get_plan_list(&self) -> ServiceResult<()> {
        todo!()
    }

    //更改计划
    pub fn update_plan(&self) -> ServiceResult<()> {
        Ok(())
    }

    pub fn delete(&self) -> ServiceResult<()> {
        Ok(())
    }
}
