use chrono::NaiveDateTime;
use rc_entity::prelude::{ProjectActiveModel, ProjectModel};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]

pub struct Link {}

#[derive(Serialize, Deserialize, Clone)]

pub struct Project {
    pub id: i32,
    pub name: String,
    pub background: Option<String>,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
    pub end_at: Option<NaiveDateTime>,
    pub link_list: Vec<Link>,
}

impl From<ProjectModel> for Project {
    fn from(value: ProjectModel) -> Self {
        let ProjectModel {
            id,
            name,
            background,
            create_at,
            update_at,
            end_at,
            ..
        } = value;

        Project {
            id,
            name,
            background,
            create_at,
            update_at,
            end_at,
            link_list: vec![],
        }
    }
}

pub struct ProjectForm {
    pub name: String,
    pub background: Option<String>,
    pub end_at: Option<NaiveDateTime>,
}

impl ProjectForm {
    pub fn get_active_model(&self) -> ProjectActiveModel {
        let active: ProjectActiveModel = Default::default();

        active
    }
}
