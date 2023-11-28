use crate::EntityResult;
use sea_orm::{ActiveModelTrait, ConnectionTrait};

use super::dto::{ProjectModelDto, ProjectOption};

pub struct ProjectDb<'a, C> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> ProjectDb<'a, C> {
    pub fn new(conn: &'a C) -> ProjectDb<C> {
        ProjectDb { conn }
    }

    pub async fn create(&self, option: ProjectOption) -> EntityResult<ProjectModelDto> {
        let model = option.into_model();

        let model = model.insert(self.conn).await?;

        let dto = ProjectModelDto::new(model);

        Ok(dto)
    }
}
