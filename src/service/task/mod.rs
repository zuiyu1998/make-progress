use crate::{Service, ServiceResult};
use migration::sea_orm::TransactionTrait;
use rc_storage::prelude::TaskStorage;

pub use rc_storage::prelude::{Task, TaskForm, TaskList, TaskParams};

pub struct TaskService<'a> {
    service: &'a Service,
}

impl<'a> TaskService<'a> {
    pub fn new(service: &'a Service) -> Self {
        Self { service }
    }

    //创建计划
    pub async fn create_task(&self, form: TaskForm) -> ServiceResult<Task> {
        let begin = self.service.conn.begin().await?;

        let storage = TaskStorage::new(&begin);

        let task = storage.create_task(form).await?;

        begin.commit().await?;

        Ok(task)
    }

    ///获取项目列表
    pub async fn get_task_list(&self, params: TaskParams) -> ServiceResult<TaskList> {
        let begin = self.service.conn.begin().await?;

        let storage = TaskStorage::new(&begin);

        let list = storage.list(params).await?;

        begin.commit().await?;

        Ok(list)
    }

    //更改计划
    pub fn update_task(&self) -> ServiceResult<()> {
        Ok(())
    }

    pub fn delete(&self) -> ServiceResult<()> {
        Ok(())
    }
}
