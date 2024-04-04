use serde::de::Error;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::process::exit;
use std::time::{SystemTime, UNIX_EPOCH};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Game {
    game_title: String,
    steam_id: u32,
    save_path: String,
    publisher: String,
    developer: String,
    saves: Vec<Save>,
}
impl Game {
    /**
    Adds a season to the Show.

    # Example
    ```
    let mut er = Game { game_title: "Elden Ring".to_string(), steam_id: 1245620, save_path: "/mnt/storage/SteamLibrary/steamapps/compatdata/1245620/".to_string(), publisher: "Bandai Namco".to_string(), developer: "FROM Software".to_string(), Saves: vec![] };
    let path:String = format!("{}/{}",base_path, er.game_title);
    er.add_save(path);
    ```
    # This adds the save to the game, to later make the backup.
    */
    #[allow(dead_code)]
    fn add_save(&mut self,_backup_path: String) {
        #[allow(unreachable_code)]
        let _new_save: Save = !todo!();
        // TODO count: should be the highest save # + 1
        // TODO backup_path: include a setting for base_path +game_name + count
        // TODO production_path: implement a save selector that supports as many formats as possible then append to parent Game's save_path
        // TODO parent_game: should be easy?
        // NOTE Should this be in epoch and converted later with a TZ defined by the user, or should it be converted now?
        let _saved_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(); 
        self.saves.push(_new_save);
    }
}
#[allow(dead_code)]
#[derive(Debug,Deserialize)]
struct Save {
    count: u8,
    backup_path: String,
    production_path: String,
    parent_game: String,
    saved_at: String, 
}


fn read_settings(setting_data: &str) -> Result<Vec<Game>, serde_json::Error> {
    let mut file = File::open(setting_data).map_err(|e| serde_json::Error::custom(e.to_string()))?;
    let mut json_data = String::new();
    file.read_to_string(&mut json_data).map_err(|e| serde_json::Error::custom(e.to_string()))?;
    let settings: Vec<Game> = serde_json::from_str(&json_data)?;
    Ok(settings)
}

fn verify_settings(settings_path: &str ) -> Vec<Game> {
    match read_settings(settings_path) {
        Ok(settings) => {
            // Return settings
            settings
        },
        Err(err) => {
            // Print error and exit
            eprintln!("Error: {}", err);
            exit(1)
        }
    }
}



fn main() {
    let games = verify_settings("./dummy.json");
    games.iter().for_each(|game| {
        println!("{}", game.game_title);
        game.saves.iter().for_each(|save|{
            println!("{}", save.count);
        })
    });
    // println!("{}",settings.into_iter());
    // println!("{:#?}", settings)
}

// Ai Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_read_settings() {
        // Create dummy JSON data
        let json_data = r#"
        [
            {
                "game_title": "Test Game",
                "steam_id": 12345,
                "save_path": "/home/user/saves/test_game",
                "publisher": "Test Publisher",
                "developer": "Test Developer",
                "saves": [
                    {
                        "count": 2,
                        "backup_path": "/backups/test_game",
                        "production_path": "/saves/test_game",
                        "parent_game": "Test Game",
                        "saved_at": "2024-03-30T15:00:00Z"
                    }
                ]
            }
        ]
        "#;

        // Create a temporary file to hold the JSON data
        let mut temp_file = tempfile::NamedTempFile::new().unwrap();
        temp_file.write_all(json_data.as_bytes()).unwrap();
        // Get the path of the temporary file
        let file_path = temp_file.path().to_str().unwrap();
        // Call read_settings() with the temporary file
        let result = read_settings(file_path);
        // Check if the result is Ok and contains the expected game title
        assert!(result.is_ok());
        let settings = result.unwrap();
        assert_eq!(settings.len(), 1);
        assert_eq!(settings[0].game_title, "Test Game");
    }

    #[test]
    fn test_verify_settings() {
        // Create dummy JSON data
        let json_data = r#"
        [
            {
                "game_title": "Test Game",
                "steam_id": 12345,
                "save_path": "/home/user/saves/test_game",
                "publisher": "Test Publisher",
                "developer": "Test Developer",
                "saves": [
                    {
                        "count": 2,
                        "backup_path": "/backups/test_game",
                        "production_path": "/saves/test_game",
                        "parent_game": "Test Game",
                        "saved_at": "2024-03-30T15:00:00Z"
                    }
                ]
            }
        ]
        "#;
        // Create a temporary file to hold the JSON data
        let mut temp_file = tempfile::NamedTempFile::new().unwrap();
        temp_file.write_all(json_data.as_bytes()).unwrap();
        // Get the path of the temporary file
        let file_path = temp_file.path().to_str().unwrap();
        // Call verify_settings() with the temporary file
        let result = verify_settings(file_path);
        // Check if the result contains the expected game title
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].game_title, "Test Game");
    }
}

