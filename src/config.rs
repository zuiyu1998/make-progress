use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::consts::{APP_NAME, APP_ORGANIZATION};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Config {
    pub system: SystemConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SystemConfig {
    pub data_dir: PathBuf,
}

impl Default for SystemConfig {
    fn default() -> Self {
        let data_dir = ProjectDirs::from("", APP_ORGANIZATION, APP_NAME)
            .unwrap()
            .data_dir()
            .to_path_buf();

        SystemConfig { data_dir }
    }
}

impl SystemConfig {
    pub fn set_data_dir(&mut self, data_dir: PathBuf) {
        self.data_dir = data_dir;
    }

    pub fn sqlite_file(&self) -> PathBuf {
        self.db_dir().join("db.sqlite")
    }

    pub fn db_dir(&self) -> PathBuf {
        self.data_dir.join("db")
    }
}
