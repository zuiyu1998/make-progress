use crate::{Service, ServiceResult};
use rc_storage::{
    chrono::NaiveDateTime,
    prelude::{ProjectStorage, ProjectStorageModel},
    sea_orm::TransactionTrait,
};

mod dto;

use dto::*;

pub struct Link {}

pub struct Project {
    pub id: i32,
    pub name: String,
    pub background: Option<String>,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
    pub end_at: Option<NaiveDateTime>,
    pub link_list: Vec<Link>,
}

impl From<ProjectStorageModel> for Project {
    fn from(value: ProjectStorageModel) -> Self {
        let ProjectStorageModel {
            id,
            name,
            background,
            create_at,
            update_at,
            end_at,
        } = value;

        Project {
            id,
            name,
            background,
            create_at,
            update_at,
            end_at,
            link_list: vec![],
        }
    }
}

pub struct ProjectService<'a> {
    service: &'a Service,
}

impl<'a> ProjectService<'a> {
    pub async fn create_project(&self, form: ProjectForm) -> ServiceResult<Project> {
        let form = form.into();
        let begin = self.service.storage.conn.begin().await?;

        let project_storage = ProjectStorage::new(&begin);

        let project = project_storage.create_project(form).await?.into();

        begin.commit().await?;

        Ok(project)
    }
}
