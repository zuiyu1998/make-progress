use crate::TauriResult;
use make_progress_core::{
    prelude::{ProjectForm, ProjectList, ProjectListParams, ProjectService},
    Service,
};
use tauri::State;

#[tauri::command]
pub async fn create_project(service: State<'_, Service>, form: ProjectForm) -> TauriResult<()> {
    let projec_service = ProjectService::new(&service);

    projec_service.create_project(form).await?;

    Ok(())
}

#[tauri::command]
pub async fn get_project_list(
    service: State<'_, Service>,
    params: ProjectListParams,
) -> TauriResult<ProjectList> {
    let projec_service = ProjectService::new(&service);

    let list = projec_service.get_project_list(params).await?;

    Ok(list)
}

#[tauri::command]
pub async fn delete_project(service: State<'_, Service>, id: i32) -> TauriResult<()> {
    let projec_service = ProjectService::new(&service);

    projec_service.delete_project(id).await?;

    Ok(())
}
