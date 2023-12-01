// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use make_progress_core::{create_service, prelude::tokio};

mod command;
mod error;

pub use error::*;

#[tokio::main]
async fn main() {
    let serivce = create_service().await;

    tauri::Builder::default()
        .manage(serivce)
        .invoke_handler(command::get_handlers())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
