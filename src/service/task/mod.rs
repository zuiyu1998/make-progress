use crate::{
    prelude::{PlanService, ProjectService},
    Service, ServiceResult,
};
use migration::sea_orm::TransactionTrait;
use rc_storage::{
    chrono::{Local, NaiveDateTime},
    prelude::{TaskStorage, TaskStorageModel},
};
use serde::{Deserialize, Serialize};

use super::{Plan, Project};

mod dto;

pub use dto::*;

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
    pub project_id: i32,
    pub plan_id: i32,
    pub remark: String,
    pub duration: i32,
    pub status: TaskStatus,
    pub real_duration: i32,
    pub start_at: Option<NaiveDateTime>,
}

impl From<TaskStorageModel> for Task {
    fn from(value: TaskStorageModel) -> Self {
        let TaskStorageModel {
            id,
            name,
            create_at,
            update_at,
            project_id,
            plan_id,
            remark,
            duration,
            real_duration,
            status,
            start_at,
            ..
        } = value;

        Task {
            id,
            name,
            create_at,
            update_at,
            project_id,
            plan_id,
            remark,
            duration,
            real_duration,
            status: TaskStatus::from(status),
            start_at,
        }
    }
}

pub struct TaskService<'a> {
    service: &'a Service,
    project: Project,
    plan: Plan,
}

impl<'a> TaskService<'a> {
    pub async fn from_project(
        project_id: i32,
        plan_id: i32,
        service: &'a Service,
    ) -> ServiceResult<Self> {
        let project_service = ProjectService::new(service);
        let project = project_service.get_project(project_id).await?;

        let plan_service = PlanService::new(project.clone(), service);

        let plan = plan_service.get_plan(plan_id).await?;

        Ok(TaskService::new(project, plan, service))
    }

    pub fn new(project: Project, plan: Plan, service: &'a Service) -> Self {
        Self {
            service,
            project,
            plan,
        }
    }

    //创建计划
    pub async fn create_task(&self, form: TaskForm) -> ServiceResult<Task> {
        let now = Local::now();

        let form = form.into_storage_form(self.project.id, self.plan.id, now.naive_local());
        let begin = self.service.storage.conn.begin().await?;

        let project_storage = TaskStorage::new(&begin);

        let project = project_storage.create_task(form).await?.into();

        begin.commit().await?;

        Ok(project)
    }

    ///获取项目列表
    pub async fn get_task_list(&self, params: TaskListParams) -> ServiceResult<TaskList> {
        let params = params.int_storage(self.project.id);

        let begin = self.service.storage.conn.begin().await?;

        let project_storage = TaskStorage::new(&begin);
        let data = project_storage.list(params).await?.into();

        begin.commit().await?;

        Ok(data)
    }

    //更改计划
    pub fn update_Task(&self) -> ServiceResult<()> {
        Ok(())
    }

    pub fn delete(&self) -> ServiceResult<()> {
        Ok(())
    }
}
