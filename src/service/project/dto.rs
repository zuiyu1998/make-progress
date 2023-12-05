use rc_storage::{
    chrono::NaiveDateTime,
    prelude::{ProjectStorageForm, ProjectStorageList, ProjectStorageListParams},
};
use serde::{Deserialize, Serialize};

use super::Project;

#[derive(Serialize, Deserialize)]
pub struct ProjectForm {
    pub name: String,
    pub background: Option<String>,
    pub end_at: Option<NaiveDateTime>,
}

impl ProjectForm {
    pub fn into_storage_form(self, now: NaiveDateTime) -> ProjectStorageForm {
        let ProjectForm {
            name,
            background,
            end_at,
        } = self;

        ProjectStorageForm {
            name,
            background,
            create_at: now.clone(),
            update_at: now,
            end_at,
        }
    }
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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
