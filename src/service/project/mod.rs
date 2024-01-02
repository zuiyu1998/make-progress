use crate::{Service, ServiceResult};
use rc_storage::{
    chrono::{Local, NaiveDateTime},
    prelude::{ProjectStorage, ProjectStorageModel},
    sea_orm::TransactionTrait,
};
use serde::{Deserialize, Serialize};

mod dto;

pub use dto::*;

#[derive(Serialize, Deserialize, Clone)]

pub struct Link {}

#[derive(Serialize, Deserialize, Clone)]

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
    pub fn new(service: &'a Service) -> Self {
        ProjectService { service }
    }

    ///获取项目详情
    pub async fn get_project(&self, project_id: i32) -> ServiceResult<Project> {
        todo!()
    }

    ///创建项目
    pub async fn create_project(&self, form: ProjectForm) -> ServiceResult<Project> {
        todo!()
    }

    ///获取项目列表
    pub async fn get_project_list(&self, params: ProjectListParams) -> ServiceResult<ProjectList> {
        todo!()
    }

    ///删除项目
    pub async fn delete_project(&self, id: i32) -> ServiceResult<()> {
        todo!()
    }
}
