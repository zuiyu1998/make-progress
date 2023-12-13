use super::model::{ActiveModel, Model, TaskEntityStatus};
use sea_orm::entity::{prelude::*, Set};

pub struct TaskModelListParams {
    pub page_size: u64,
    pub page: u64,
}

pub struct TaskModelList {
    pub data: Vec<TaskModelDto>,
    pub total: u64,
}

pub struct TaskModelDto {
    pub id: i32,
    pub name: String,
    pub project_id: i32,
    pub plan_id: i32,
    pub create_at: ChronoDateTime,
    pub update_at: ChronoDateTime,
    pub start_at: Option<ChronoDateTime>,
    pub remark: String,
    pub duration: i32,
    pub status: TaskEntityStatus,
    pub real_duration: i32,
}

impl TaskModelDto {
    pub fn new(task: Model) -> Self {
        let Model {
            id,
            name,
            create_at,
            update_at,
            project_id,
            plan_id,
            remark,
            duration,
            status,
            real_duration,
            start_at,
            ..
        } = task;

        TaskModelDto {
            id,
            name,
            create_at,
            update_at,
            project_id,
            plan_id,
            remark,
            duration,
            status,
            real_duration,
            start_at,
        }
    }
}

#[derive(Default)]
pub struct TaskOption {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub create_at: Option<ChronoDateTime>,
    pub update_at: Option<ChronoDateTime>,
    pub project_id: Option<i32>,
    pub plan_id: Option<i32>,
    pub remark: Option<String>,
    pub duration: Option<i32>,
    pub real_duration: Option<i32>,
    pub status: Option<TaskEntityStatus>,
    pub start_at: Option<ChronoDateTime>,
}

impl TaskOption {
    pub fn into_model(self) -> ActiveModel {
        let mut active: ActiveModel = Default::default();

        assert_eq!(true, self.project_id.is_some());
        assert_eq!(true, self.plan_id.is_some());

        active.project_id = Set(self.project_id.unwrap());
        active.plan_id = Set(self.plan_id.unwrap());

        if let Some(status) = self.status {
            active.status = Set(status);
        }

        if let Some(real_duration) = self.real_duration {
            active.real_duration = Set(real_duration);
        }

        if let Some(duration) = self.duration {
            active.duration = Set(duration);
        }

        if let Some(remark) = self.remark {
            active.remark = Set(remark);
        }

        if let Some(update_at) = self.update_at {
            active.update_at = Set(update_at);
        }

        if let Some(create_at) = self.create_at {
            active.create_at = Set(create_at);
        }

        if let Some(name) = self.name {
            active.name = Set(name);
        }

        if let Some(id) = self.id {
            active.id = Set(id);
        }

        active
    }
}
