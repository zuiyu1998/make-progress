use crate::EntityResult;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};

use super::dto::{ProjectModelDto, ProjectOption};
use super::ProjectActiveModel;

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

    pub async fn delete(&self, id: i32) -> EntityResult<()> {
        let mut model: ProjectActiveModel = Default::default();

        model.id = Set(id);

        model.delete(self.conn).await?;

        Ok(())
    }
}
