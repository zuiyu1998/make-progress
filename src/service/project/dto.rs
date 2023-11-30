use rc_storage::{
    chrono::NaiveDateTime,
    prelude::{ProjectStorageForm, ProjectStorageList, ProjectStorageListParams},
};

use super::Project;

pub struct ProjectForm {
    pub name: String,
    pub background: String,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
    pub end_at: Option<NaiveDateTime>,
}

impl From<ProjectForm> for ProjectStorageForm {
    fn from(value: ProjectForm) -> Self {
        let ProjectForm {
            name,
            background,
            create_at,
            update_at,
            end_at,
        } = value;

        ProjectStorageForm {
            name,
            background,
            create_at,
            update_at,
            end_at,
        }
    }
}

pub struct ProjectListParams {
    pub page_size: u64,
    pub page: u64,
}

impl From<ProjectListParams> for ProjectStorageListParams {
    fn from(value: ProjectListParams) -> Self {
        ProjectStorageListParams {
            page_size: value.page_size,
            page: value.page,
        }
    }
}

pub struct ProjectList {
    pub data: Vec<Project>,
    pub total: u64,
    pub page_size: u64,
    pub page: u64,
    pub has_next: bool,
}

impl From<ProjectStorageList> for ProjectList {
    fn from(value: ProjectStorageList) -> Self {
        ProjectList {
            total: value.total,
            page: value.page,
            page_size: value.page_size,
            has_next: value.has_next,
            data: value
                .data
                .into_iter()
                .map(|item| Project::from(item))
                .collect::<Vec<Project>>(),
        }
    }
}
