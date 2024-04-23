use std::{path::PathBuf, process::exit};
use serde::{Deserialize, Serialize};
extern crate fs_extra;
use fs_extra::dir::{copy, CopyOptions};
use std::time::Instant;

use super::test_create_dir;


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
impl Save {
    #[allow(dead_code)]
    pub fn backup(&mut self) {
        let mut coptions = CopyOptions::new();
        coptions.overwrite = true;
        coptions.copy_inside = true;
        let start = Instant::now();
        if let Err(err) = test_create_dir(&self.backup_path) {
            eprintln!("Could not create path for backing up due to {}", err);
            exit(1);
        }
        match copy(self.production_path.clone(), self.backup_path.clone(), &coptions){
            Ok(_) => println!("\x1b[32mSuccessfully backed up \x1b[34m{}\x1b[0m in \x1b[36m{:.2?}", self.parent_game, start.elapsed()),
            Err(err) => eprintln!("Failed to back up {} due to {:?}", self.parent_game, err.kind)
        }
    }
    // TODO: Before overwriting the production_path, copy that to /tmp in case of errors
    #[allow(dead_code)]
    pub fn restore(&mut self) {
        
        let mut coptions = CopyOptions::new();
        coptions.overwrite = true;
        coptions.content_only = true;
        let start = Instant::now();
        if let Err(err) = test_create_dir(&self.production_path) {
            eprintln!("Could not create path for backing up due to {}", err);
            exit(1);
        }
        match copy(self.backup_path.clone(), self.production_path.clone(), &coptions){
            Ok(_) => println!("\x1b[32mSuccessfully restored \x1b[34m{}\x1b[0m in \x1b[36m{:.2?}", self.parent_game, start.elapsed()),
            Err(err) => eprintln!("Failed to back up {} due to {:?}", self.parent_game, err.kind)
        }

        
    }
}
