use crate::TauriResult;
use make_progress_core::{
    prelude::{TaskForm, TaskListParams, TaskService},
    Service,
};
use tauri::State;

#[tauri::command]
pub async fn create_task(service: State<'_, Service>, form: TaskForm) -> TauriResult<()> {
    let plan_service = TaskService::new(&service);

    plan_service.create_task(form).await?;

    Ok(())
}

#[tauri::command]
pub async fn get_task_list(service: State<'_, Service>, params: TaskListParams) -> TauriResult<()> {
    let task_service = TaskService::new(&service);

    task_service.get_task_list(params).await?;

    Ok(())
}
