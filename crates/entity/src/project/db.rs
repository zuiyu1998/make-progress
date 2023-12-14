use crate::{EntityKind, EntityResult};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, Set,
};

use super::dto::{ProjectModelDto, ProjectModelList, ProjectModelListParams, ProjectOption};
use super::{ProjectActiveModel, ProjectColumn, ProjectEntity};

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

    pub async fn find(&self, id: i32) -> EntityResult<Option<ProjectModelDto>> {
        let model = ProjectEntity::find()
            .filter(ProjectColumn::Id.eq(id))
            .one(self.conn)
            .await?
            .map(|model| ProjectModelDto::new(model));

        Ok(model)
    }

    pub async fn get(&self, id: i32) -> EntityResult<ProjectModelDto> {
        let model = self.find(id).await?;
        if model.is_none() {
            return Err(EntityKind::ProjectNotFound.into());
        }

        Ok(model.unwrap())
    }

    pub async fn delete(&self, id: i32) -> EntityResult<()> {
        let mut model: ProjectActiveModel = Default::default();

        model.id = Set(id);

        model.delete(self.conn).await?;

        Ok(())
    }

    pub async fn list(&self, params: ProjectModelListParams) -> EntityResult<ProjectModelList> {
        let sql = ProjectEntity::find();

        let total = sql.clone().count(self.conn).await?;

        let sql = sql.paginate(self.conn, params.page_size);

        let models = sql
            .fetch_page(params.page)
            .await?
            .into_iter()
            .map(|item| ProjectModelDto::new(item))
            .collect::<Vec<ProjectModelDto>>();

        Ok(ProjectModelList {
            total,
            data: models,
        })
    }
}
