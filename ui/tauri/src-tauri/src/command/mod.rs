pub mod plan;
pub mod project;

pub fn get_handlers() -> Box<dyn Fn(tauri::Invoke<tauri::Wry>) + Send + Sync> {
    Box::new(tauri::generate_handler![
        project::create_project,
        project::get_project_list,
        project::delete_project,
        plan::create_plan,
        plan::get_plan_list,
    ])
}
