use crate::TauriResult;
use make_progress_core::{
    prelude::{Plan, PlanForm, PlanService},
    Service,
};
use tauri::State;

#[tauri::command]
pub async fn create_plan(service: State<'_, Service>, form: PlanForm) -> TauriResult<()> {
    let service = PlanService::new(&service);

    service.create_plan(form).await?;

    Ok(())
}

#[tauri::command]
pub async fn get_plan_list(service: State<'_, Service>) -> TauriResult<()> {
    Ok(())
}
