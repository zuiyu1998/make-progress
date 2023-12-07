use rc_storage::chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlanForm {
    pub name: String,
    pub dead_at: NaiveDateTime,
}
