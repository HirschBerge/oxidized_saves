use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Save {
    // u8 Should be enough, but testing easily surpases this lol
    pub count: u16,
    pub backup_path: PathBuf,
    pub production_path: PathBuf,
    pub parent_game: String,
    pub saved_at: String,
}
// impl Save {
//     // TODO: Implment method to backup a specific save. 
//     fn backup(self) {
//         !todo!();
//         
//     }
//     // TODO: Implment method to restore a specific save.
//     fn restore(self) {
//         !todo!();
//         
//     }
// }
