use serde::de::{DeserializeOwned, Error};
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[allow(dead_code)]
#[derive(Debug, Deserialize, PartialEq)]
struct Settings {
    save_base_path: PathBuf,
    color_scheme: String,
    delete_on_restore: bool,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Game {
    game_title: String,
    steam_id: u32,
    save_path: PathBuf,
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
    fn add_save(&mut self, _backup_path: String, settings_path: &String) {
        // NOTE Is this the most efficient manner to get the count?
        let _count = self
            .saves
            .iter()
            .max_by_key(|save| save.count)
            .map(|save| save.count + 1)
            .unwrap_or(0);
        // NOTE parent_game: helps backup_path
        let _parent_game = self.game_title.clone();
        // NOTE  backup_path: simply a path made up of the path defined in your settings, the name of the game, and the count of the settings.
        let _backup_path: PathBuf = PathBuf::from(format!("{}/{}/{}", settings_path, &_parent_game, &_count));
        // TODO production_path: implement a save selector that supports as many formats as possible then append to parent Game's save_path
        // NOTE Should this be in epoch and converted later with a TZ defined by the user, or should it be converted now?
        let _saved_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        #[allow(unreachable_code)]
        let _new_save: Save = !todo!();
        self.saves.push(_new_save);
    }
}
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Save {
    count: u8,
    backup_path: PathBuf,
    production_path: PathBuf,
    parent_game: String,
    saved_at: String,
}

fn read_conf<T>(conf_data: PathBuf) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
{
    let mut file = File::open(conf_data).map_err(|e| serde_json::Error::custom(e.to_string()))?;
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)
        .map_err(|e| serde_json::Error::custom(e.to_string()))?;
    let settings: T = serde_json::from_str(&json_data)?;
    Ok(settings)
}

// Define a function to verify and return the configuration
fn verify_conf<T>(conf_path: PathBuf) -> T
where
    T: DeserializeOwned,
{
    match read_conf(conf_path) {
        Ok(conf) => conf,
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}

fn main() {
    // Get the user's home directory
    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => {
            println!("Unable to determine home directory.");
            return;
        }
    };
    // Append the relative path to the user's home directory
    let settings_file = home_dir.join(".config/oxi/oxi.json");
    let prog_settings: &Settings = &verify_conf::<Vec<Settings>>(settings_file)[0];
    println!("{:?}", prog_settings);
    let games: Vec<Game> = verify_conf(PathBuf::from("./dummy.json"));
    games.iter().for_each(|game| {
        println!("{}\n{:?}", game.game_title, game.save_path);
        if let Some(max_save) = game.saves.iter().max_by_key(|save| save.count) {
            println!("Total saves: {}", max_save.count);
        }
    });
    // println!("{}",settings.into_iter());
    // println!("{:#?}", settings)
}

// Ai Tests
#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;
    use std::io::Write;

    #[test]
    fn fuzz_bad_settings() {
        // Create a temporary directory for testing
        let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");

        // Define the content of the settings file
        let settings_content = r#"[
            {
                "save_base_path": "/path/to/save",
                "color_scheme": "dark",
                "delete_on_restore": false
            }
        ]"#;

        // Define the path to the settings file
        let settings_file_path = temp_dir.path().join("oxi_bad.json");

        // Write the settings content to the file
        std::fs::write(&settings_file_path, settings_content)
            .expect("Failed to write settings file");

        // Mock the home directory to the temporary directory
        let home_dir = temp_dir.path().to_path_buf();

        // Call the code under test
        let prog_settings = {
            // Append the relative path to the user's home directory
            let settings_file = home_dir.join("oxi_bad.json");
            &verify_conf::<Vec<Settings>>(settings_file)[0]
        };

        // Define the expected settings
        let expected_settings = Settings {
            save_base_path: PathBuf::from("/path/to/save"),
            color_scheme: String::from("dark"),
            delete_on_restore: false,
        };

        // Verify that the actual settings match the expected settings
        assert_eq!(prog_settings, &expected_settings);        // Verify that the actual settings match the expected settings
        assert_eq!(prog_settings.save_base_path, expected_settings.save_base_path);
    }
    #[test]
    fn test_read_conf() {
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
        let file_path = PathBuf::from(temp_file.path().to_str().unwrap());
        // Call read_settings() with the temporary file
        let result = read_conf(file_path);
        // Check if the result is Ok and contains the expected game title
        assert!(result.is_ok());
        let settings: Vec<Game> = result.unwrap();
        assert_eq!(settings.len(), 1);
        assert_eq!(settings[0].game_title, "Test Game");
    }

    #[test]
    fn test_verify_conf() {
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
        let file_path = PathBuf::from(temp_file.path().to_str().unwrap());
        // Call verify_settings() with the temporary file
        let result: Vec<Game> = verify_conf(file_path);
        // Check if the result contains the expected game title
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].game_title, "Test Game");
        assert_eq!(result[0].publisher, "Test Publisher");
        assert_eq!(result[0].saves[0].count, 2);
    }

    #[test]
    fn fuzz_simulate_bad_data() {
        // TEST with other break cases.
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
                        "parent_game": "Test Game,
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
        let file_path = PathBuf::from(temp_file.path().to_str().unwrap());
        // Test to see if it returns an error or not. If it returns an error, this test is a successful fuzz and passes.
        assert_matches!(read_conf::<Vec<Game>>(file_path), Err(_)); // Check if an error is returned
    }
}
