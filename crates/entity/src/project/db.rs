use crate::EntityResult;
use sea_orm::{ActiveModelTrait, ConnectionTrait};

use super::dto::{ProjectModelDto, ProjectOption};

pub struct ProjectDb<C> {
    conn: C,
}

impl<C: ConnectionTrait> ProjectDb<C> {
    pub async fn create(&self, option: ProjectOption) -> EntityResult<ProjectModelDto> {
        let model = option.into_model();

        let model = model.insert(&self.conn).await?;

        let dto = ProjectModelDto::new(model);

        Ok(dto)
    }
}
