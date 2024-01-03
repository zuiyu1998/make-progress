use crate::{prelude::Task, StorageResult};
use rc_entity::{
    prelude::{ProjectActiveModel, ProjectEntity},
    sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait, PaginatorTrait, Set},
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

    pub async fn find_project(&self, id: i32) -> StorageResult<Option<Project>> {
        let model = ProjectEntity::find_by_id(id)
            .one(self.conn)
            .await?
            .and_then(|item| Some(Project::from(item)));

        Ok(model)
    }

    pub fn update_project(&self) -> StorageResult<()> {
        todo!()
    }

    pub async fn list(&self, params: ProjectParams) -> StorageResult<ProjectList> {
        let sql = ProjectEntity::find();

        let paginator = sql.paginate(self.conn, params.page_size as u64);

        let list = paginator.fetch_page(params.page as u64).await?;

        let mut has_next = true;

        if list.len() < params.page_size as usize {
            has_next = false;
        }

        let project_list = ProjectList {
            data: list
                .into_iter()
                .map(|item| Project::from(item))
                .collect::<Vec<Project>>(),
            has_next,
            page: params.page,
            page_size: params.page_size,
        };

        Ok(project_list)
    }
}
