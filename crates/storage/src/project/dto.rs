use chrono::NaiveDateTime;
use rc_entity::prelude::{ProjectModelDto, ProjectModelListParams, ProjectOption};

pub struct ProjectStorageForm {
    pub name: String,
    pub background: Option<String>,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
    pub end_at: Option<NaiveDateTime>,
}

impl ProjectStorageForm {
    pub fn into_option(self) -> ProjectOption {
        let mut option = ProjectOption::default();

        option.name = Some(self.name);
        option.background = self.background;
        option.create_at = Some(self.create_at);
        option.update_at = Some(self.update_at);
        option.end_at = self.end_at;

        option
    }
}

pub struct ProjectStorageUpdate {}

pub struct ProjectStorageModel {
    pub id: i32,
    pub name: String,
    pub background: Option<String>,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
    pub end_at: Option<NaiveDateTime>,
}

pub struct ProjectStorageList {
    pub data: Vec<ProjectStorageModel>,
    pub total: u64,
    pub page_size: u64,
    pub page: u64,
    pub has_next: bool,
}

impl From<ProjectModelDto> for ProjectStorageModel {
    fn from(value: ProjectModelDto) -> Self {
        let ProjectModelDto {
            id,
            name,
            background,
            create_at,
            update_at,
            end_at,
        } = value;

        ProjectStorageModel {
            id,
            name,
            background,
            create_at,
            update_at,
            end_at,
        }
    }
}

pub struct ProjectStorageListParams {
    pub page_size: u64,
    pub page: u64,
}

impl From<ProjectStorageListParams> for ProjectModelListParams {
    fn from(value: ProjectStorageListParams) -> Self {
        ProjectModelListParams {
            page_size: value.page_size,
            page: value.page,
        }
    }
}
