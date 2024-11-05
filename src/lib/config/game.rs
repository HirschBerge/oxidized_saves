use crate::config::gen_home;
use crate::config::save::Save;
use crate::config::steam;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub game_title: String,
    pub game_id: u32,
    pub save_path: Option<PathBuf>,
    pub publisher: Option<String>,
    pub developer: Option<String>,
    pub saves: Option<Vec<Save>>,
    pub thumbnail: Vec<PathBuf>,
}
impl Game {
    pub fn print_info(&self) {
        println!(
            "\x1b[34mTitle\x1b[31m: {}\n\x1b[34mApp ID\x1b[35m: {}\n\x1b[34mPath to Icon:",
            self.game_title, self.game_id,
        );
        for thumb in &self.thumbnail {
            println!("\t\x1b[32m{}\x1b[0m", thumb.to_string_lossy());
        }
    }
    /**
    # Usecase
    Generates a `Option<PathBuf>` that represents the Proton C Drive which can be used as a starting location when selecting a save path.
    */
    #[allow(dead_code)]
    fn find_compatdata(&self) -> Option<PathBuf> {
        let home_dir = gen_home().expect("All OSes should have a home directory.");
        let steam_lib: PathBuf = home_dir.join(".local/share/Steam/config/libraryfolders.vdf");
        let steam_paths = steam::extract_steampath(steam_lib);
        for path in steam_paths {
            // NOTE: drilling further into proton path due to too many symlinks
            let combined_path = path.join(format!(
                "compatdata/{}/pfx/drive_c/pfx/drive_c/users/steamuser/",
                self.game_id
            ));
            if let Ok(_meta) = fs::metadata(&combined_path) {
                return Some(combined_path);
            }
        }
        Some(home_dir)
    }
    // https://docs.rs/fs_extra/latest/fs_extra/dir/fn.copy.html
    // TEST: Write exhaustive tests
    #[allow(dead_code)]
    fn backup_all_saves(self) {
        match self.saves {
            Some(saves_list) => {
                for mut save in saves_list {
                    save.backup();
                }
            }
            None => {
                println!("No saves found.");
            }
        }
    }
    #[allow(dead_code)]
    fn restore_all_saves(self) {
        match self.saves {
            Some(saves_list) => {
                for mut save in saves_list {
                    save.restore();
                }
            }
            None => {
                println!("No saves found.");
            }
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
        game_id: 1245620,
        save_path: Some(PathBuf::from("/mnt/storage/SteamLibrary/steamapps/compatdata/1245620/")),
        publisher: Some("Bandai Namco".to_string()),
        developer: Some("FROM Software".to_string()),
        saves: vec![].into(),
        thumbnail: vec![].into(),
    };
    er.add_save(prod_path, &settings_path);
    ```
    # This adds the save to the game, to later make the backup.
    */
    pub fn add_save(&mut self, production_path: PathBuf, settings_path: &Path) {
        // NOTE: Is this the most efficient manner to get the count?
        let count = self
            .saves
            .as_ref() // NOTE: Avoids consuming
            .into_iter()
            .flatten()
            .max_by_key(|save| save.count)
            .map(|save| save.count + 1)
            .unwrap_or(0);
        // NOTE: parent_game: helps backup_path
        let parent_game = self.game_title.clone();
        // NOTE:  backup_path: simply a path made up of the path defined in your settings, the name of the game, and the count of the settings.
        let backup_path: PathBuf = PathBuf::from(format!(
            "{}/{}/{}",
            settings_path.to_str().unwrap_or("/home/user/"),
            &parent_game,
            &count
        ));
        // NOTE: Should this be in epoch and converted later with a TZ defined by the user, or should it be converted now?
        let saved_at = Local::now()
            .naive_local()
            .format("%Y-%m-%dT%H:%M:%SZ")
            .to_string();
        let new_save: Save = Save {
            count,
            backup_path,
            production_path,
            parent_game,
            saved_at,
        };
        if let Some(saves) = &mut self.saves {
            saves.push(new_save);
        } else {
            eprintln!("There are so saves for {}", self.game_title);
        }
    }
}
