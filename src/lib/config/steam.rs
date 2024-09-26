use crate::config::game::Game;
use core::panic;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
// TODO: Make better interoptability with `Game`, or merge `SteamGame` and `Game` together
pub struct SteamGame {
    game_name: String,
    app_id: u64,
    thumbnail: Vec<PathBuf>,
}
impl SteamGame {
    #[allow(dead_code)]
    fn convert_to_game(&self) -> Game {
        todo!();
        // TODO:
        // 1. Determine if Game struct instance with identical Game.game_title exists.
        //   a. If SteamGame content is identical, skip
        //   b. If exists and not identical, update information app_id -> steam_id, thumbnail -> thumbnail
        //   c. Else, set default info for save_path, publisher, developer, saves and then do 1.b.
        //
    }
    fn print_info(&self) {
        println!(
            "\x1b[34mTitle\x1b[31m: {}\n\x1b[34mApp ID\x1b[35m: {}\n\x1b[34mPath to Icon:",
            self.game_name, self.app_id,
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
        let steam_paths = extract_steampath(steam_lib);
        for path in steam_paths {
            // NOTE: drilling further into proton path due to too many symlinks
            let combined_path = path.join(format!(
                "compatdata/{}/pfx/drive_c/pfx/drive_c/users/steamuser/",
                self.app_id
            ));
            if let Ok(_meta) = fs::metadata(&combined_path) {
                return Some(combined_path);
            }
        }
        Some(home_dir)
    }
}
/// # Description:
/// Contains a list of banned game titles (entirely non-game steam/proton-related tools) and, given a title, returns a `bool` based on if they are on the ban list
fn filter_banned_games(title: &Option<String>) -> bool {
    let banned_terms = ["Proton", "Steam Linux", "Steamworks"];
    if let Some(ref t) = title {
        banned_terms.iter().any(|&banned| t.contains(banned))
    } else {
        false
    }
}
/**
 Parses the contents of an .acf file and extracts relevant information for a `SteamGame` instance.

 # Arguments

  `thumb_path` - A reference to the directory containing game thumbnails.
  `reader` - A buffered reader for the .acf file.

 # Returns

 A tuple containing the extracted information: `(app_id, thumbnail, game_name)`.
 Each item in the tuple is an `Option`:
 - `app_id`: The Steam application ID.
 - `thumbnail`: The path to the game thumbnail.
 - `game_name`: The name of the game.

 # Examples

 ```
 use std::path::Path;
 use std::fs::File;
 use std::io::BufReader;
 use oxi::config::steam::parse_acf_files;
 let thumb_path = Path::new("./Cargo.lock");
 let file = File::open("./Cargo.toml").expect("Failed to open file");
 let reader = BufReader::new(file);
 let (app_id, thumbnail, game_name) = parse_acf_files(thumb_path, reader);
 ```
*/
pub fn parse_acf_files(
    thumb_path: &Path,
    reader: BufReader<File>,
) -> (Option<u64>, Option<Vec<PathBuf>>, Option<String>) {
    // Initiate variables for SteamGame
    let mut app_id: Option<u64> = None;
    let mut thumbnails: Option<Vec<PathBuf>> = Some(Vec::new());
    let mut game_name: Option<String> = None;

    // Loop over the lines in the acf file
    for line in reader.lines().map_while(Result::ok) {
        // Pull out the app_id and generate the path for the thumbnail
        if line.contains("\"appid\"") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                app_id = parts[1]
                    .trim_matches(|c| c == '"' || c == '\'')
                    .parse()
                    .ok();
                if let Some(id) = app_id {
                    let endings = [
                        "_library_600x900.jpg",
                        "_icon.jpg",
                        "_logo.png",
                        "_library_hero.jpg",
                        "_library_hero_blur.jpg",
                        "_header.jpg",
                    ];
                    if let Some(ref mut thumbs) = thumbnails {
                        for ending in &endings {
                            let full_path =
                                format!("{}/{}{}", thumb_path.to_string_lossy(), id, ending);
                            match Path::new(&full_path).exists() {
                                true => thumbs.push(Path::new(&full_path).to_path_buf()),
                                false => {
                                    let home_dir = gen_home()
                                        .expect("All OSes should have a home directory!??");
                                    thumbs.push(
                                        Path::new(&home_dir.join(".config/oxi/placeholder.png"))
                                            .to_path_buf(),
                                    )
                                }
                            }
                        }
                    }
                }
            }
        // Get the game_name
        } else if line.contains("\"name\"") {
            let parts: Vec<&str> = line.split('"').collect();
            if parts.len() >= 3 {
                game_name = Some(parts[3].to_string());
            }
        }
    }

    if let Some(ref thumbs) = thumbnails {
        if thumbs.is_empty() {
            thumbnails = None;
        }
    }

    (app_id, thumbnails, game_name)
}

/** Returns a vector of `SteamGame` instances parsed from .acf files in the specified directory.

 # Arguments

  `directory_path` - A reference to the directory containing .acf files.
  `thumb_path` - A reference to the directory containing game thumbnails.

 # Returns

 An `Option` containing a vector of `SteamGame` instances if successful, or `None` if an error occurs.

 # Examples

 ```
 use std::path::Path;
 use oxi::config::steam::return_steamgames;
 let directory_path = Path::new("/path/to/directory");
 let thumb_path = Path::new("/path/to/thumbnails");
 let steam_games = return_steamgames(directory_path, thumb_path);
 ```
**/
pub fn return_steamgames(directory_path: &Path, thumb_path: &Path) -> Option<Vec<SteamGame>> {
    let mut steamgames: Vec<SteamGame> = Vec::new();
    // Iterate over the entries in the directory
    match std::fs::read_dir(directory_path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                // Check if the entry is a file with a .acf extension
                match (entry.path().extension(), entry.file_type()) {
                    (Some(ext), Ok(file_type)) if ext == "acf" && file_type.is_file() => {
                        let acf_file = File::open(entry.path());
                        if let Ok(the_file) = acf_file {
                            let reader = BufReader::new(the_file);
                            let (app_id, thumbnail, game_name) =
                                parse_acf_files(thumb_path, reader);
                            if filter_banned_games(&game_name) {
                                continue;
                            }
                            // As long as they all exist, create the struct instance
                            if let (Some(app_id), Some(thumbnail), Some(game_name)) =
                                (app_id, thumbnail, game_name)
                            {
                                let game = SteamGame {
                                    app_id,
                                    thumbnail,
                                    game_name,
                                };
                                steamgames.push(game);
                            }
                        } else {
                            eprintln!("Failed to open file: {:?}", entry.path());
                        }
                    }
                    _ => {}
                }
            }
            Some(steamgames)
        }
        Err(_) => {
            eprintln!("Failed to read directory.");
            None
        }
    }
}
/**
Parses the contents of an .acf file and extracts relevant information for a `SteamGame` instance.

# Arguments

- `thumb_path` - A reference to the directory containing game thumbnails.
- `reader` - A buffered reader for the .acf file.

# Returns

A tuple containing the extracted information: `(app_id, thumbnail, game_name)`.
Each item in the tuple is an `Option`:
- `app_id`: The Steam application ID.
- `thumbnail`: The path to the game thumbnail.
- `game_name`: The name of the game.

# Examples
**/
fn read_file(path: PathBuf) -> File {
    // Open the file
    match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Failed to open file.");
            panic!();
        }
    }
}

/**
Parses a file to extract Steam library paths.

# Arguments

- `path` - The path to the file containing Steam library information.

# Returns

A vector containing the extracted Steam library paths.

# Examples
```
use std::path::PathBuf;
use oxi::config::steam::extract_steampath;
let path = PathBuf::from("./Cargo.toml");
let libraries = extract_steampath(path);
```
*/
pub fn extract_steampath(path: PathBuf) -> Vec<PathBuf> {
    // Create a vector to store the extracted data
    let mut extracted_libraries: Vec<PathBuf> = Vec::new();
    let file = read_file(path);
    // Read the file .line by line and extract data
    let reader = BufReader::new(file);
    for line in reader.lines().map_while(Result::ok) {
        if line.contains("path") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let cleaned_path = parts[1].trim_matches(|c| c == '"' || c == '\'');
            extracted_libraries.push(PathBuf::from(format!("{}/steamapps/", cleaned_path)));
        }
    }
    extracted_libraries
}

fn combine_steampaths(extracted_libraries: Vec<PathBuf>, thumb_path: PathBuf) -> Vec<SteamGame> {
    let mut combined_steamgames: Vec<SteamGame> = Vec::new();
    for libraries in &extracted_libraries {
        if let Some(steamgames) = return_steamgames(libraries, &thumb_path) {
            combined_steamgames.extend(steamgames);
        } else {
            // Handle the case when return_steamgames returns None
            eprintln!("Failed to retrieve Steam games for {:?}", libraries);
        }
    }
    combined_steamgames
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

pub fn discover_steamgames(verbose: bool) -> Vec<SteamGame> {
    let home_dir = gen_home().expect("All OSes should have a home directory!??");
    let steam_lib: PathBuf = home_dir.join(".local/share/Steam/config/libraryfolders.vdf");
    let steam_thumb: PathBuf = home_dir.join(".local/share/Steam/appcache/librarycache");
    let steam_paths = extract_steampath(steam_lib.clone());
    let mut libraries = combine_steampaths(steam_paths, steam_thumb);
    println!(
        "\x1b[34mWe have found \x1b[31m{}\x1b[34m Steam games on your system!",
        libraries.len()
    );
    if verbose {
        libraries.sort_by(|a, b| a.game_name.cmp(&b.game_name));
        libraries.iter().for_each(|game| game.print_info());
    }
    libraries
}
