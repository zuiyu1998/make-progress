use chrono::NaiveDateTime;
use rc_entity::prelude::{TaskEntityStatus, TaskModelDto, TaskOption};

pub struct TaskStorageForm {
    pub name: String,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
    pub project_id: i32,
    pub plan_id: i32,
    pub duration: i32,
    pub real_duration: i32,
    pub status: TaskStorageStatus,
    pub start_at: Option<NaiveDateTime>,
}

pub enum TaskStorageStatus {
    Start,
    End,
    Pause,
    Playing,
}

impl From<TaskStorageStatus> for TaskEntityStatus {
    fn from(value: TaskStorageStatus) -> Self {
        match value {
            TaskStorageStatus::Start => TaskEntityStatus::Start,
            TaskStorageStatus::End => TaskEntityStatus::End,
            TaskStorageStatus::Pause => TaskEntityStatus::Pause,
            TaskStorageStatus::Playing => TaskEntityStatus::Playing,
        }
    }
}

impl From<TaskEntityStatus> for TaskStorageStatus {
    fn from(value: TaskEntityStatus) -> Self {
        match value {
            TaskEntityStatus::Start => TaskStorageStatus::Start,
            TaskEntityStatus::End => TaskStorageStatus::End,
            TaskEntityStatus::Pause => TaskStorageStatus::Pause,
            TaskEntityStatus::Playing => TaskStorageStatus::Playing,
        }
    }
}

impl TaskStorageForm {
    pub fn into_option(self) -> TaskOption {
        let mut option = TaskOption::default();

        option.start_at = self.start_at;
        option.name = Some(self.name);
        option.create_at = Some(self.create_at);
        option.update_at = Some(self.update_at);
        option.project_id = Some(self.project_id);
        option.plan_id = Some(self.plan_id);
        option.duration = Some(self.duration);
        option.real_duration = Some(self.real_duration);
        option.status = Some(self.status.into());

        option
    }
}

pub struct TaskStorageUpdate {}

pub struct TaskStorageModel {
    pub id: i32,
    pub name: String,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
    pub project_id: i32,
    pub plan_id: i32,
    pub remark: String,
    pub duration: i32,
    pub status: TaskStorageStatus,
    pub real_duration: i32,
    pub start_at: Option<NaiveDateTime>,
}

pub struct TaskStorageList {
    pub data: Vec<TaskStorageModel>,
    pub total: u64,
    pub page_size: u64,
    pub page: u64,
    pub has_next: bool,
}

impl From<TaskModelDto> for TaskStorageModel {
    fn from(value: TaskModelDto) -> Self {
        let TaskModelDto {
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
        } = value;

        TaskStorageModel {
            id,
            name,
            create_at,
            update_at,
            project_id,
            real_duration,
            duration,
            remark,
            status: TaskStorageStatus::from(status),
            plan_id,
            start_at,
        }
    }
}

pub struct TaskStorageListParams {
    pub page_size: u64,
    pub page: u64,
    pub project_id: Option<i32>,
}
