use rc_entity::prelude::{TaskColumn, TaskEntity};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TaskEntity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TaskColumn::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TaskColumn::Name).string().not_null())
                    .col(ColumnDef::new(TaskColumn::CreateAt).date_time().not_null())
                    .col(ColumnDef::new(TaskColumn::UpdateAt).date_time().not_null())
                    .col(ColumnDef::new(TaskColumn::ProjectId).integer().not_null())
                    .col(ColumnDef::new(TaskColumn::StartAt).date_time())
                    .col(ColumnDef::new(TaskColumn::PlanId).integer().not_null())
                    .col(ColumnDef::new(TaskColumn::Status).string().not_null())
                    .col(ColumnDef::new(TaskColumn::Duration).integer().not_null())
                    .col(
                        ColumnDef::new(TaskColumn::RealDuration)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TaskColumn::IsDelete)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TaskColumn::IsEnable)
                            .boolean()
                            .default(true)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TaskColumn::Remark)
                            .string()
                            .default("")
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TaskEntity).to_owned())
            .await
    }
}
