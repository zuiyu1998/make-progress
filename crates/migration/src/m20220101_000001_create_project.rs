use rc_entity::prelude::{ProjectColumn, ProjectEntity};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProjectEntity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProjectColumn::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ProjectColumn::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(ProjectColumn::Background).string())
                    .col(
                        ColumnDef::new(ProjectColumn::CreateAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectColumn::UpdateAt)
                            .date_time()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ProjectColumn::EndAt).date_time())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProjectEntity).to_owned())
            .await
    }
}
