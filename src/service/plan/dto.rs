use super::Plan;
use rc_storage::{
    chrono::NaiveDateTime,
    prelude::{PlanStorageForm, PlanStorageList, PlanStorageListParams},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlanForm {
    pub name: String,
    pub dead_at: NaiveDateTime,
}

impl PlanForm {
    pub fn into_storage_form(self, project_id: i32, now: NaiveDateTime) -> PlanStorageForm {
        let PlanForm { name, dead_at } = self;

        PlanStorageForm {
            name,
            create_at: now.clone(),
            update_at: now,
            dead_at,
            project_id,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PlanListParams {
    pub page_size: u64,
    pub page: u64,
}

impl PlanListParams {
    pub fn int_storage(self, project_id: i32) -> PlanStorageListParams {
        PlanStorageListParams {
            page_size: self.page_size,
            page: self.page,
            project_id: Some(project_id),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PlanList {
    pub data: Vec<Plan>,
    pub total: u64,
    pub page_size: u64,
    pub page: u64,
    pub has_next: bool,
}

impl From<PlanStorageList> for PlanList {
    fn from(value: PlanStorageList) -> Self {
        PlanList {
            total: value.total,
            page: value.page,
            page_size: value.page_size,
            has_next: value.has_next,
            data: value
                .data
                .into_iter()
                .map(|item| Plan::from(item))
                .collect::<Vec<Plan>>(),
        }
    }
}
