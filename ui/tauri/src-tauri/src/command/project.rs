use crate::TauriResult;
use make_progress_core::{prelude::ProjectService, Service};
use tauri::State;

#[tauri::command]
pub async fn create_project(service: State<'_, Service>) -> TauriResult<()> {
    let projec_service = ProjectService::new(&service);

    Ok(())
}

#[tauri::command]
pub async fn get_project_list(service: State<'_, Service>) -> TauriResult<()> {
    let projec_service = ProjectService::new(&service);

    Ok(())
}

#[tauri::command]
pub async fn delete_project(service: State<'_, Service>, id: i32) -> TauriResult<()> {
    let projec_service = ProjectService::new(&service);

    projec_service.delete_project(id).await?;

    Ok(())
}
