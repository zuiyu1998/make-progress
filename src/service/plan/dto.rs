use rc_storage::{chrono::NaiveDateTime, prelude::PlanStorageForm};
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
