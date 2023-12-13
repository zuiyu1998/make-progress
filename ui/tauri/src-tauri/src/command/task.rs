use crate::TauriResult;
use make_progress_core::{
    prelude::{TaskForm, TaskService},
    Service,
};
use tauri::State;

#[tauri::command]
pub async fn create_task(service: State<'_, Service>, form: TaskForm) -> TauriResult<()> {
    let plan_service = TaskService::new(&service);

    plan_service.create_task(form).await?;

    Ok(())
}
