use chrono::NaiveDateTime;
use rc_entity::prelude::{TaskModel, TaskModelStatus};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TaskParams {
    pub project_id: Option<i32>,
    pub plan_id: Option<i32>,
    pub page: i32,
    pub page_size: i32,
}

pub struct TaskList {
    pub data: Vec<Task>,
    pub has_next: bool,
    pub page: i32,
    pub page_size: i32,
}

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

impl From<TaskModel> for Task {
    fn from(value: TaskModel) -> Self {
        let TaskModel {
            id,
            name,
            project_id,
            plan_id,
            create_at,
            update_at,
            start_at,
            remark,
            duration,
            status,
            real_duration,
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
            status: status.into(),
            real_duration,
            start_at,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum TaskStatus {
    Start,
    End,
    Pause,
    Playing,
}

impl From<TaskModelStatus> for TaskStatus {
    fn from(value: TaskModelStatus) -> Self {
        match value {
            TaskModelStatus::End => TaskStatus::End,
            TaskModelStatus::Pause => TaskStatus::Pause,
            TaskModelStatus::Playing => TaskStatus::Playing,
            TaskModelStatus::Start => TaskStatus::Start,
        }
    }
}
