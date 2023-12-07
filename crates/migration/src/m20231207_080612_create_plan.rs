use rc_entity::prelude::{PlanColumn, PlanEntity};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PlanEntity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PlanColumn::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PlanColumn::Name).string().not_null())
                    .col(ColumnDef::new(PlanColumn::CreateAt).date_time().not_null())
                    .col(ColumnDef::new(PlanColumn::UpdateAt).date_time().not_null())
                    .col(ColumnDef::new(PlanColumn::ProjectId).integer().not_null())
                    .col(ColumnDef::new(PlanColumn::DeadAt).date_time())
                    .col(
                        ColumnDef::new(PlanColumn::IsDelete)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlanColumn::IsEnable)
                            .boolean()
                            .default(true)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlanColumn::Remark)
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
            .drop_table(Table::drop().table(PlanEntity).to_owned())
            .await
    }
}
