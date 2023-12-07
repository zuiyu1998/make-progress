use crate::{prelude::ProjectService, Service, ServiceResult};
use rc_storage::chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

use super::Project;

mod dto;

pub use dto::*;

#[derive(Serialize, Deserialize)]
pub struct Plan {
    pub id: i32,
    pub name: String,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
    pub dead_at: NaiveDateTime,
    pub project_id: i32,
}

pub struct PlanService<'a> {
    service: &'a Service,
    project: Project,
}

impl<'a> PlanService<'a> {
    pub async fn from_project(project_id: i32, service: &'a Service) -> ServiceResult<Self> {
        let project_service = ProjectService::new(service);

        let project = project_service.get_project(project_id).await?;

        Ok(PlanService::new(project, service))
    }

    pub fn new(project: Project, service: &'a Service) -> Self {
        Self { service, project }
    }

    //创建计划
    pub fn create_plan(&self, form: PlanForm) -> ServiceResult<()> {
        Ok(())
    }

    //更改计划
    pub fn update_plan(&self) -> ServiceResult<()> {
        Ok(())
    }

    pub fn delete(&self) -> ServiceResult<()> {
        Ok(())
    }
}
