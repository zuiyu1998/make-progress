use super::Task;
use rc_storage::{
    chrono::NaiveDateTime,
    prelude::{TaskStorageForm, TaskStorageList, TaskStorageListParams, TaskStorageStatus},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TaskForm {
    pub name: String,
    pub start_at: Option<i64>,
    pub duration: i32,
    pub project_id: i32,
    pub plan_id: i32,
}

#[derive(Serialize, Deserialize)]
pub enum TaskStatus {
    Start,
    End,
    Pause,
    Playing,
}

impl From<TaskStatus> for TaskStorageStatus {
    fn from(value: TaskStatus) -> Self {
        match value {
            TaskStatus::End => TaskStorageStatus::End,
            TaskStatus::Pause => TaskStorageStatus::Pause,
            TaskStatus::Playing => TaskStorageStatus::Playing,
            TaskStatus::Start => TaskStorageStatus::Start,
        }
    }
}

impl From<TaskStorageStatus> for TaskStatus {
    fn from(value: TaskStorageStatus) -> Self {
        match value {
            TaskStorageStatus::End => TaskStatus::End,
            TaskStorageStatus::Pause => TaskStatus::Pause,
            TaskStorageStatus::Playing => TaskStatus::Playing,
            TaskStorageStatus::Start => TaskStatus::Start,
        }
    }
}

impl TaskForm {
    pub fn into_storage_form(self, now: NaiveDateTime) -> TaskStorageForm {
        let TaskForm {
            name,
            start_at,
            duration,
            plan_id,
            project_id,
        } = self;

        TaskStorageForm {
            name,
            create_at: now.clone(),
            update_at: now,
            project_id,
            plan_id,
            duration,
            real_duration: 0,
            status: TaskStorageStatus::Start,
            start_at: start_at.and_then(|millis| NaiveDateTime::from_timestamp_millis(millis)),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TaskListParams {
    pub page_size: u64,
    pub page: u64,
    pub project_id: Option<i32>,
    pub plan_id: Option<i32>,
}

impl TaskListParams {
    pub fn int_storage(self) -> TaskStorageListParams {
        TaskStorageListParams {
            page_size: self.page_size,
            page: self.page,
            project_id: self.project_id,
            plan_id: self.plan_id,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TaskList {
    pub data: Vec<Task>,
    pub total: u64,
    pub page_size: u64,
    pub page: u64,
    pub has_next: bool,
}

impl From<TaskStorageList> for TaskList {
    fn from(value: TaskStorageList) -> Self {
        TaskList {
            total: value.total,
            page: value.page,
            page_size: value.page_size,
            has_next: value.has_next,
            data: value
                .data
                .into_iter()
                .map(|item| Task::from(item))
                .collect::<Vec<Task>>(),
        }
    }
}
