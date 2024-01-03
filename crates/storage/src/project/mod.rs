use crate::StorageResult;
use rc_entity::{
    prelude::ProjectActiveModel,
    sea_orm::{ActiveModelTrait, ConnectionTrait, Set},
};

mod dto;

pub use dto::*;

pub struct ProjectStorage<'a, C> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> ProjectStorage<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        ProjectStorage { conn }
    }

    pub async fn delete(&self, id: i32) -> StorageResult<Project> {
        let mut active: ProjectActiveModel = Default::default();

        active.id = Set(id);
        active.is_delete = Set(true);

        let model = active.update(self.conn).await?;

        Ok(Project::from(model))
    }

    pub async fn create_project(&self, form: ProjectForm) -> StorageResult<Project> {
        let active = form.get_active_model();

        let model = active.insert(self.conn).await?;

        Ok(Project::from(model))
    }

    pub async fn find_project(&self, id: i32) -> StorageResult<()> {
        todo!()
    }

    pub fn update_project(&self) -> StorageResult<()> {
        todo!()
    }

    pub async fn list(&self) -> StorageResult<()> {
        todo!()
    }
}
