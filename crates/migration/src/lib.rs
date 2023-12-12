pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_project;
mod m20231207_080612_create_plan;
mod m20231212_072946_create_task;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_project::Migration),
            Box::new(m20231207_080612_create_plan::Migration),
            Box::new(m20231212_072946_create_task::Migration),
        ]
    }
}
