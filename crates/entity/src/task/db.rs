use crate::{EntityKind, EntityResult};
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set};

use super::dto::{TaskModelDto, TaskOption};
use super::{TaskActiveModel, TaskColumn, TaskEntity};

pub struct TaskDb<'a, C> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> TaskDb<'a, C> {
    pub fn new(conn: &'a C) -> TaskDb<C> {
        TaskDb { conn }
    }

    pub async fn create(&self, option: TaskOption) -> EntityResult<TaskModelDto> {
        let model = option.into_model();

        let model = model.insert(self.conn).await?;

        let dto = TaskModelDto::new(model);

        Ok(dto)
    }

    pub async fn find(&self, id: i32) -> EntityResult<Option<TaskModelDto>> {
        let model = TaskEntity::find()
            .filter(TaskColumn::Id.eq(id))
            .one(self.conn)
            .await?
            .map(|model| TaskModelDto::new(model));

        Ok(model)
    }

    pub async fn get(&self, id: i32) -> EntityResult<TaskModelDto> {
        let model = self.find(id).await?;
        if model.is_none() {
            return Err(EntityKind::TaskNotFound.into());
        }

        Ok(model.unwrap())
    }

    pub async fn delete(&self, id: i32) -> EntityResult<()> {
        let mut model: TaskActiveModel = Default::default();

        model.id = Set(id);

        model.delete(self.conn).await?;

        Ok(())
    }
}
