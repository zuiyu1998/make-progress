use chrono::NaiveDateTime;
use rc_entity::prelude::{PlanActiveModel, PlanModel};
use serde::{Deserialize, Serialize};

pub struct PlanList {
    pub data: Vec<Plan>,
    pub has_next: bool,
    pub page: i32,
    pub page_size: i32,
}

pub struct PlanParams {
    pub page: i32,
    pub page_size: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Plan {
    pub id: i32,
    pub name: String,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
    pub dead_at: NaiveDateTime,
    pub project_id: i32,
}

impl From<PlanModel> for Plan {
    fn from(value: PlanModel) -> Self {
        let PlanModel {
            id,
            name,
            create_at,
            update_at,
            dead_at,
            project_id,
            ..
        } = value;

        Plan {
            id,
            name,
            create_at,
            update_at,
            dead_at,
            project_id,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PlanForm {
    pub name: String,
    pub dead_at: NaiveDateTime,
    pub project_id: i32,
}

impl PlanForm {
    pub fn get_active_model(&self) -> PlanActiveModel {
        let active: PlanActiveModel = Default::default();

        active
    }
}
