pub mod game;
pub mod steam;
pub mod save;
use std::{fs::File, io::Read, path::{Path, PathBuf}};
use serde::{de::{DeserializeOwned, Error}, Serialize};

pub fn write_conf<T>(conf: Vec<T>, pth: &Path)
// pub fn write_conf<T>(conf: Vec<T>, pth: PathBuf -> serde_json::Result<()>) 
where
    T: Serialize,
{
    if let Ok(out_file) = File::create(pth){
        if let Err(err) = serde_json::to_writer(out_file, &conf){
            eprintln!("Oh noes: {}",err);
        }
    }else if let Err(err) = File::create(pth) {
        eprintln!("Error creating file {:?}: {}", pth, err);
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
    match read_conf(conf_path) {
        Ok(conf) => conf,
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}
