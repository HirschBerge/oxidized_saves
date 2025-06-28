pub mod game;
pub mod save;
pub mod steam;
use serde::{
    de::{DeserializeOwned, Error},
    Deserialize, Serialize,
};
use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigEntry {
    pub save_base_path: String,
    pub game_conf_path: String,
    pub color_scheme: String,
    pub delete_on_restore: bool,
}

fn test_create_dir(path: &PathBuf) -> Result<(), io::Error> {
    // Check if the directory already exists
    if !path.exists() {
        // Create the directory and necessary parent directories
        fs::create_dir_all(path)?;
        println!("Created directory: {:?}", path);
    } else {
        println!("{:?} already exists!", path);
    }

    Ok(())
}
pub fn write_conf<T>(conf: Vec<T>, pth: &Path)
// pub fn write_conf<T>(conf: Vec<T>, pth: PathBuf -> serde_json::Result<()>)
where
    T: Serialize,
{
    if let Ok(out_file) = File::create(pth) {
        if let Err(err) = serde_json::to_writer(out_file, &conf) {
            eprintln!("Oh noes: {}", err);
        }
    } else if let Err(err) = File::create(pth) {
        eprintln!("Error creating file {:?}: {}", pth, err);
    }
}
pub fn create_config() {
    let default_settings = r#"[
        {
            "save_base_path": "/Documents/Saves",
            "game_conf_path": ".config/oxi/",
            "color_scheme": "dark",
            "delete_on_restore": true
        }
    ]"#;
    match serde_json::from_str::<Vec<ConfigEntry>>(default_settings) {
        Ok(config_entries) => {
            let user_home = gen_home().expect("All computers should have a home dir.");
            let config_path = user_home.join(".config/oxi/");
            if !PathBuf::from(&config_path).exists() {
                if let Err(e) = fs::create_dir_all(&config_path) {
                    eprintln!("  Error creating save directory {:?}: {}", config_path, e);
                }
            }
            let config_file = config_path.join("oxi.json");
            write_conf(config_entries, config_file.as_path());
            let game_config_file = config_path.join("conf.json");
            let content = "[]";
            let mut file = match File::create(game_config_file.clone()) {
                Ok(f) => f, // If successful, assign the File handle to 'f'
                Err(e) => {
                    // If an error occurred, print an error message and exit the function
                    eprintln!(
                        "Error creating file '{:?}': {}",
                        game_config_file.as_os_str(),
                        e
                    );
                    return; // Exit main function
                }
            };

            // Attempt to write content to the file
            match file.write_all(content.as_bytes()) {
                Ok(_) => {
                    // '_': We don't care about the successful value (which is ())
                    println!(
                        "Successfully wrote '{}' to '{:?}'",
                        content, game_config_file
                    );
                }
                Err(e) => {
                    // If an error occurred during writing, print an error message
                    eprintln!(
                        "Error writing to file '{:?}': {}",
                        game_config_file.as_os_str(),
                        e
                    );
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to deserialize default settings: {}", e);
        }
    }
}

pub fn read_conf<T>(conf_data: PathBuf) -> Result<T, serde_json::Error>
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
pub fn verify_conf<T>(conf_path: PathBuf) -> T
where
    T: DeserializeOwned,
{
    match read_conf(conf_path.clone()) {
        Ok(conf) => conf,
        Err(err) => {
            eprintln!("Error on {:?}: {}", conf_path.into_os_string(), err);
            std::process::exit(1);
        }
    }
}

/**
# Usecase
Just Generates an expanded ~/
*/
pub fn gen_home() -> Option<PathBuf> {
    match dirs::home_dir() {
        Some(path) => Some(path),
        None => {
            println!("Unable to determine home directory.");
            None
        }
    }
}
