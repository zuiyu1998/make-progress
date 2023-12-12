use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "pb_task")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub project_id: i32,
    pub plan_id: i32,
    pub create_at: ChronoDateTime,
    pub update_at: ChronoDateTime,
    pub start_at: Option<ChronoDateTime>,
    pub is_delete: bool,
    pub is_enable: bool,
    pub remark: String,
    pub duration: i32,
    pub status: TaskEntityStatus,
    pub real_duration: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(None)")]
pub enum TaskEntityStatus {
    #[sea_orm(string_value = "Start")]
    Start,
    #[sea_orm(string_value = "End")]
    End,
    #[sea_orm(string_value = "Pause")]
    Pause,
    #[sea_orm(string_value = "Playing")]
    Playing,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
