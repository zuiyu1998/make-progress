use crate::StorageResult;
use rc_entity::{
    prelude::{TaskColumn, TaskEntity},
    sea_orm::{
        ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter,
    },
};

mod dto;

pub use dto::*;

pub struct TaskStorage<'a, C> {
    conn: &'a C,
}

impl<'a, C: ConnectionTrait> TaskStorage<'a, C> {
    pub fn new(conn: &'a C) -> Self {
        TaskStorage { conn }
    }

    pub async fn delete(&self, id: i32) -> StorageResult<()> {
        Ok(())
    }

    pub async fn create_task(&self, form: TaskForm) -> StorageResult<Task> {
        let active = form.get_active_model();

        let model = active.insert(self.conn).await?;

        Ok(Task::from(model))
    }

    pub async fn find_task(&self, _id: i32) -> StorageResult<()> {
        Ok(())
    }

    pub fn update_task(&self) -> StorageResult<()> {
        todo!()
    }

    pub async fn list(&self, params: TaskParams) -> StorageResult<TaskList> {
        let mut sql = TaskEntity::find();

        if let Some(project_id) = params.project_id {
            sql = sql.filter(TaskColumn::ProjectId.eq(project_id));
        }

        if let Some(plan_id) = params.plan_id {
            sql = sql.filter(TaskColumn::PlanId.eq(plan_id));
        }

        let paginator = sql.paginate(self.conn, params.page_size as u64);

        let list = paginator.fetch_page(params.page as u64).await?;

        let mut has_next = true;

        if list.len() < params.page_size as usize {
            has_next = false;
        }

        let task_list = TaskList {
            data: list
                .into_iter()
                .map(|item| Task::from(item))
                .collect::<Vec<Task>>(),
            has_next,
            page: params.page,
            page_size: params.page_size,
        };

        Ok(task_list)
    }
}
