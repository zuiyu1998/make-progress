use crate::{EntityKind, EntityResult};
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set};

use super::dto::{PlanModelDto, PlanOption};
use super::{PlanActiveModel, PlanColumn, PlanEntity};

pub struct PlanDb<'a, C> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> PlanDb<'a, C> {
    pub fn new(conn: &'a C) -> PlanDb<C> {
        PlanDb { conn }
    }

    pub async fn create(&self, option: PlanOption) -> EntityResult<PlanModelDto> {
        let model = option.into_model();

        let model = model.insert(self.conn).await?;

        let dto = PlanModelDto::new(model);

        Ok(dto)
    }

    pub async fn find(&self, id: i32) -> EntityResult<Option<PlanModelDto>> {
        let model = PlanEntity::find()
            .filter(PlanColumn::Id.eq(id))
            .one(self.conn)
            .await?
            .map(|model| PlanModelDto::new(model));

        Ok(model)
    }

    pub async fn get(&self, id: i32) -> EntityResult<PlanModelDto> {
        let model = self.find(id).await?;
        if model.is_none() {
            return Err(EntityKind::PlanNotFound.into());
        }

        Ok(model.unwrap())
    }

    pub async fn delete(&self, id: i32) -> EntityResult<()> {
        let mut model: PlanActiveModel = Default::default();

        model.id = Set(id);

        model.delete(self.conn).await?;

        Ok(())
    }
}
