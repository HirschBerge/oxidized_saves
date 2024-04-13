mod settings;
use std::path::PathBuf;
use settings::Settings;
mod config;
use config::{game::Game, steam::discover_steamgames, verify_conf, write_conf};


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
    let settings_file = &home_dir.join(".config/oxi/oxi.json");
    let prog_settings: &Settings = &verify_conf::<Vec<Settings>>(settings_file.to_path_buf())[0];
    // println!("{:?}", prog_settings);
    let game_conf_path = PathBuf::from(format!("{}/{}conf.json",home_dir.to_string_lossy(), prog_settings.game_conf_path.to_string_lossy()));
    let mut games: Vec<Game> = verify_conf(game_conf_path.clone());
    games[0].add_save(PathBuf::from("/mnt/storage/SteamLibrary/steamapps/compatdata/292030/"), &prog_settings.save_base_path);
    write_conf(games, &game_conf_path);
    discover_steamgames();
}

// Ai Tests
#[cfg(test)]
mod tests {
    use std::io::Write;
    use crate::config::read_conf;
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn fuzz_bad_settings() {
        // Create a temporary directory for testing
        let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");

        // Define the content of the settings file
        let settings_content = r#"[
            {
                "save_base_path": "/path/to/save",
                "game_conf_path": ".config/oxi/",
                "color_scheme": "dark",
                "delete_on_restore": false
            }
        ]"#;

        // Define the path to the settings file
        let settings_file_path = temp_dir.path().join("oxi_bad.json");

        // Write the settings content to the file
        std::fs::write(settings_file_path, settings_content)
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
            game_conf_path: PathBuf::from(".config/oxi/"),
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
