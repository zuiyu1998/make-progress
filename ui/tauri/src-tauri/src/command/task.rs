use crate::TauriResult;
use make_progress_core::{
    prelude::{TaskForm, TaskService},
    Service,
};
use tauri::State;

#[tauri::command]
pub async fn create_task(
    service: State<'_, Service>,
    project_id: i32,
    plan_id: i32,
    form: TaskForm,
) -> TauriResult<()> {
    let plan_service = TaskService::from_project(project_id, plan_id, &service).await?;

    plan_service.create_task(form).await?;

    Ok(())
}
