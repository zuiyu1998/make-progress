use crate::TauriResult;
use make_progress_core::{
    prelude::{PlanForm, PlanListParams, PlanService},
    Service,
};
use tauri::State;

#[tauri::command]
pub async fn create_plan(
    service: State<'_, Service>,
    project_id: i32,
    form: PlanForm,
) -> TauriResult<()> {
    let plan_service = PlanService::from_project(project_id, &service).await?;

    plan_service.create_plan(form).await?;

    Ok(())
}

#[tauri::command]
pub async fn get_plan_list(
    service: State<'_, Service>,
    project_id: i32,
    params: PlanListParams,
) -> TauriResult<()> {
    let plan_service = PlanService::from_project(project_id, &service).await?;

    plan_service.get_plan_list(params).await?;

    Ok(())
}
