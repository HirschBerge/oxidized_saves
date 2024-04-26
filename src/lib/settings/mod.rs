use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Settings {
    pub save_base_path: PathBuf,
    pub game_conf_path: PathBuf,
    pub color_scheme: String,
    pub delete_on_restore: bool,
}
