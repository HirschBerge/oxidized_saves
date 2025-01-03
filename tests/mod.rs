// Ai Tests
#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use oxi::config::{game::Game, read_conf, verify_conf};
    use oxi::settings::Settings;
    use std::io::Write;
    use std::path::PathBuf;

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
        assert_eq!(prog_settings, &expected_settings); // Verify that the actual settings match the expected settings
        assert_eq!(
            prog_settings.save_base_path,
            expected_settings.save_base_path
        );
    }
    #[test]
    fn test_read_conf() {
        // Create dummy JSON data
        let json_data = r#"
        [
            {
                "game_title": "Test Game",
                "game_id": 12345,
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
                ],
                "thumbnail": []
            }
        ]
        "#;
        // Create a temporary file to hold the JSON data
        let mut temp_file = tempfile::NamedTempFile::new().expect("A valid temp file.");
        temp_file
            .write_all(json_data.as_bytes())
            .expect("If this is failing. Why isn't the line above failing first?");
        // Get the path of the temporary file
        let file_path = PathBuf::from(
            temp_file
                .path()
                .to_str()
                .expect("If this is failing, the two lines above should panic too."),
        );
        // Call read_settings() with the temporary file
        let result = read_conf(file_path);
        // Check if the result is Ok and contains the expected game title
        assert!(result.is_ok());
        let settings: Vec<Game> = result
            .expect("If this is panicing, the assert above should have triggered first. Why?");
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
                "game_id": 12345,
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
                ],
                "thumbnail": [ ]
            }
        ]
        "#;
        // Create a temporary file to hold the JSON data
        let mut temp_file = tempfile::NamedTempFile::new()
            .expect("Why would a temp file not be able to be created?");
        temp_file
            .write_all(json_data.as_bytes())
            .expect("If this panics, why didn't the line above?");
        // Get the path of the temporary file
        let file_path = PathBuf::from(
            temp_file
                .path()
                .to_str()
                .expect("This is the third expect. This shouldn't be the one panicking."),
        );
        // Call verify_settings() with the temporary file
        let result: Vec<Game> = verify_conf(file_path);
        // Check if the result contains the expected game title
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].game_title, "Test Game");
        assert_eq!(result[0].publisher.as_deref(), Some("Test Publisher"));
        assert_eq!(result[0].saves.as_ref().unwrap()[0].count, 2);
    }

    #[test]
    fn fuzz_simulate_bad_data() {
        // TEST: with other break cases.
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
                ],
                "thumbnail": []
            }
        ]
        "#;
        // Create a temporary file to hold the JSON data
        let mut temp_file =
            tempfile::NamedTempFile::new().expect("Why does creating a tempfile panicK?");
        temp_file
            .write_all(json_data.as_bytes())
            .expect("A temp file should never panic");
        // Get the path of the temporary file
        let file_path = PathBuf::from(
            temp_file
                .path()
                .to_str()
                .expect("There are two expects before this. WTF?"),
        );
        // Test to see if it returns an error or not. If it returns an error, this test is a successful fuzz and passes.
        assert_matches!(read_conf::<Vec<Game>>(file_path), Err(_)); // Check if an error is returned
    }
}
