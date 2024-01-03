use crate::{Service, ServiceResult};
use rc_storage::{
    prelude::{ProjectForm, ProjectStorage},
    sea_orm::TransactionTrait,
};

pub use rc_storage::prelude::Project;

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
        let begin = self.service.conn.begin().await?;

        let storage = ProjectStorage::new(&begin);

        let project = storage.create_project(form).await?;

        begin.commit().await?;

        Ok(project)
    }

    ///获取项目列表
    pub async fn get_project_list(&self) -> ServiceResult<()> {
        todo!()
    }

    ///删除项目
    pub async fn delete_project(&self, id: i32) -> ServiceResult<()> {
        todo!()
    }
}
