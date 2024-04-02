use serde::de::Error;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::process::exit;

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
            // Process settings
            settings
        },
        Err(err) => {
            // Handle error
            eprintln!("Error: {}", err);
            exit(1)
            // Optionally, you can exit the program or take other actions based on the error.
        }
    }
}

fn main() {
    let settings = verify_settings("./test.json");
    println!("{:#?}", settings)
}

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

