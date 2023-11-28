use rc_storage::{chrono::NaiveDateTime, prelude::ProjectStorageForm, sea_orm::Related};

pub struct ProjectForm {
    pub name: String,
    pub background: String,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
    pub end_at: Option<NaiveDateTime>,
}

impl From<ProjectForm> for ProjectStorageForm {
    fn from(value: ProjectForm) -> Self {
        let ProjectForm {
            name,
            background,
            create_at,
            update_at,
            end_at,
        } = value;

        ProjectStorageForm {
            name,
            background,
            create_at,
            update_at,
            end_at,
        }
    }
}
