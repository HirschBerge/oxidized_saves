use std::path::{Path, PathBuf};
use crate::config::save::Save;
use chrono::Local;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub game_title: String,
    pub steam_id: u32,
    pub save_path: PathBuf,
    pub publisher: String,
    pub developer: String,
    pub saves: Vec<Save>,
}
impl Game {
    // https://docs.rs/fs_extra/latest/fs_extra/dir/fn.copy.html
    // TEST: This still needs to be tested.
    #[allow(dead_code)]
    fn backup_all_saves(self) {
        for mut save in self.saves {
            save.backup();
        }
    }
    // TODO: Implement method to restore all Saves attached to this game
    #[allow(dead_code)]
    fn restore_all_saves(self) {
        for mut save in self.saves {
            save.restore();
        }
    }
    /**
    Adds a season to the Show.

    # Example
    ```
    use std::path::Path;
    use chrono::Local;
    use oxi::config::game::Game;
    use std::path::PathBuf;
    
    let settings_path: PathBuf = PathBuf::from("Documents/saves");
    let prod_path: PathBuf = PathBuf::from("/mnt/games");
    let mut er = Game {
        game_title: "Elden Ring".to_string(),
        steam_id: 1245620,
        save_path: PathBuf::from("/mnt/storage/SteamLibrary/steamapps/compatdata/1245620/"),
        publisher: "Bandai Namco".to_string(),
        developer: "FROM Software".to_string(),
        saves: vec![],
    };
    er.add_save(prod_path, &settings_path);
    ```
    # This adds the save to the game, to later make the backup.
    */
    pub fn add_save(&mut self, production_path: PathBuf, settings_path: &Path) {
        // NOTE: Is this the most efficient manner to get the count?
        let count = self
            .saves
            .iter()
            .max_by_key(|save| save.count)
            .map(|save| save.count + 1)
            .unwrap_or(0);
        // NOTE: parent_game: helps backup_path
        let parent_game = self.game_title.clone();
        // NOTE:  backup_path: simply a path made up of the path defined in your settings, the name of the game, and the count of the settings.
        let backup_path: PathBuf = PathBuf::from(format!("{}/{}/{}", settings_path.to_str().unwrap_or("/home/user/"), &parent_game, &count));
        // NOTE: Should this be in epoch and converted later with a TZ defined by the user, or should it be converted now?
        let saved_at = Local::now().naive_local().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        let new_save: Save = Save {
        count,
        backup_path,
        production_path,
        parent_game,
        saved_at,
    };
        self.saves.push(new_save);
    }
}
