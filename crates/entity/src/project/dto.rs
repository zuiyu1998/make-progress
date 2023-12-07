use super::model::{ActiveModel, Model};
use sea_orm::entity::{prelude::*, Set};

pub struct ProjectModelDto {
    pub id: i32,
    pub name: String,
    pub background: Option<String>,
    pub create_at: ChronoDateTime,
    pub update_at: ChronoDateTime,
    pub end_at: Option<ChronoDateTime>,
}

impl ProjectModelDto {
    pub fn new(project: Model) -> Self {
        let Model {
            id,
            name,
            background,
            create_at,
            update_at,
            end_at,
            ..
        } = project;

        ProjectModelDto {
            id,
            name,
            background,
            create_at,
            update_at,
            end_at,
        }
    }
}

#[derive(Default)]
pub struct ProjectOption {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub background: Option<String>,
    pub create_at: Option<ChronoDateTime>,
    pub update_at: Option<ChronoDateTime>,
    pub end_at: Option<ChronoDateTime>,
}

impl ProjectOption {
    pub fn into_model(self) -> ActiveModel {
        let mut active: ActiveModel = Default::default();

        active.end_at = Set(self.end_at);

        if let Some(update_at) = self.update_at {
            active.update_at = Set(update_at);
        }

        if let Some(create_at) = self.create_at {
            active.create_at = Set(create_at);
        }

        active.background = Set(self.background);

        if let Some(name) = self.name {
            active.name = Set(name);
        }

        if let Some(id) = self.id {
            active.id = Set(id);
        }

        active
    }
}
