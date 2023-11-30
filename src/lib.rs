use config::Config;
use migration::{Migrator, MigratorTrait};
use rc_storage::{sea_orm::Database, Storage};
use std::{fs, sync::Arc};

mod config;
mod consts;
mod error;
mod service;

pub use error::*;
pub use service::Service;

pub mod prelude {
    pub use crate::service::*;
    pub use tokio;
}

pub async fn create_service() -> Service {
    let config = Arc::new(Config::default());

    initialization(config.clone());

    let sqlite_file = config.system.sqlite_file();
    let sqlite_path = sqlite_file.to_str().unwrap();
    let conn_str = format!("sqlite:///{}", sqlite_path);

    let conn = Database::connect(&conn_str)
        .await
        .expect("databas connect error.");

    Migrator::up(&conn, None)
        .await
        .expect("create migration error.");

    let storage = Storage::new(conn);

    Service { storage }
}

pub fn initialization(config: Arc<Config>) {
    let db_dir = config.system.db_dir();

    fs::create_dir_all(db_dir).expect("create db dir.");

    if !config.system.sqlite_file().exists() {
        fs::File::create(config.system.sqlite_file()).expect("create sqlite file.");
    }

    tracing_subscriber::fmt().init();
}
