use super::model::{ActiveModel, Model};
use sea_orm::entity::{prelude::*, Set};

pub struct PlanModelListParams {
    pub page_size: u64,
    pub page: u64,
}

pub struct PlanModelList {
    pub data: Vec<PlanModelDto>,
    pub total: u64,
}

pub struct PlanModelDto {
    pub id: i32,
    pub name: String,
    pub create_at: ChronoDateTime,
    pub update_at: ChronoDateTime,
    pub dead_at: ChronoDateTime,
    pub project_id: i32,
}

impl PlanModelDto {
    pub fn new(project: Model) -> Self {
        let Model {
            id,
            name,
            create_at,
            update_at,
            dead_at,
            project_id,
            ..
        } = project;

        PlanModelDto {
            id,
            name,
            create_at,
            update_at,
            dead_at,
            project_id,
        }
    }
}

#[derive(Default)]
pub struct PlanOption {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub create_at: Option<ChronoDateTime>,
    pub update_at: Option<ChronoDateTime>,
    pub dead_at: Option<ChronoDateTime>,
    pub project_id: Option<i32>,
}

impl PlanOption {
    pub fn into_model(self) -> ActiveModel {
        let mut active: ActiveModel = Default::default();

        assert_eq!(true, self.project_id.is_some());

        active.project_id = Set(self.project_id.unwrap());

        if let Some(dead_at) = self.dead_at {
            active.dead_at = Set(dead_at);
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
