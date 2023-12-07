use chrono::NaiveDateTime;
use rc_entity::prelude::{PlanModelDto, PlanOption};

pub struct PlanStorageForm {
    pub name: String,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
    pub dead_at: NaiveDateTime,
}

impl PlanStorageForm {
    pub fn into_option(self) -> PlanOption {
        let mut option = PlanOption::default();

        option.name = Some(self.name);
        option.create_at = Some(self.create_at);
        option.update_at = Some(self.update_at);
        option.dead_at = Some(self.dead_at);

        option
    }
}

pub struct PlanStorageUpdate {}

pub struct PlanStorageModel {
    pub id: i32,
    pub name: String,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
    pub dead_at: NaiveDateTime,
}

pub struct PlanStorageList {
    pub data: Vec<PlanStorageModel>,
    pub total: u64,
    pub page_size: u64,
    pub page: u64,
    pub has_next: bool,
}

impl From<PlanModelDto> for PlanStorageModel {
    fn from(value: PlanModelDto) -> Self {
        let PlanModelDto {
            id,
            name,
            create_at,
            update_at,
            dead_at,
        } = value;

        PlanStorageModel {
            id,
            name,
            create_at,
            update_at,
            dead_at,
        }
    }
}

pub struct PlanStorageListParams {
    pub page_size: u64,
    pub page: u64,
}
