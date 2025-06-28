use crate::config::{game::Game, gen_home};
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    path::{Path, PathBuf},
};

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{char, multispace0, space0, space1},
    combinator::map,
    sequence::{delimited, separated_pair},
    IResult,
};

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
 Parses the contents of an .acf file and extracts relevant information for a `Game` instance.

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
 let library = Path::new("./Cargo.lock");
 let file = File::open("./Cargo.toml").expect("Failed to open file");
 let reader = BufReader::new(file);
 let (install_dir, app_id, thumbnail, game_name) = parse_acf_files(library, thumb_path, reader);
 ```
*/
pub fn parse_acf_files(
    library: &Path,
    thumb_path: &Path,
    reader: BufReader<File>,
) -> (
    Option<PathBuf>,
    Option<u32>,
    Option<Vec<PathBuf>>,
    Option<String>,
) {
    // Initiate variables for Game
    let mut thumbnails: Option<Vec<PathBuf>> = Some(Vec::new());
    let mut game_name: Option<String> = None;
    let mut app_id = 0;
    let mut install_dir = PathBuf::new();

    // Loop over the lines in the acf file
    for line in reader.lines().map_while(Result::ok) {
        // Pull out the app_id and generate the path for the thumbnail
        if line.contains("\"installdir\"") {
            install_dir = match parse_install_path(line.as_str(), library) {
                Ok((_, path)) => path,
                Err(e) => {
                    eprintln!("Failed to parse out install_dir: {}", e);
                    std::process::exit(1);
                }
            };
        } else if line.contains("\"appid\"") {
            (_, app_id) = match parse_app_id(line.as_str()) {
                Ok(id) => id,
                Err(e) => {
                    eprintln!("Could not parse app_id into u32: {}", e);
                    std::process::exit(1);
                }
            };
            let endings = [
                "library_600x900.jpg",
                "icon.jpg",
                "logo.png",
                "library_hero.jpg",
                "library_hero_blur.jpg",
                "header.jpg",
            ];
            if let Some(ref mut thumbs) = thumbnails {
                endings.iter().for_each(|ending| {
                    let full_path =
                        format!("{}/{}/{}", thumb_path.to_string_lossy(), app_id, ending);
                    match Path::new(&full_path).exists() {
                        true => thumbs.push(Path::new(&full_path).to_path_buf()),
                        false => {
                            let home_dir =
                                gen_home().expect("All OSes should have a home directory!??");
                            thumbs.push(
                                Path::new(&home_dir.join(".config/oxi/placeholder.png"))
                                    .to_path_buf(),
                            )
                        }
                    }
                })
            }
        // Get the game_name
        } else if line.contains("\"name\"") {
            game_name = match parse_name(line.as_str()) {
                Ok((_, name)) => Some(name),
                Err(e) => {
                    eprintln!("Could not parse game name: {}", e);
                    std::process::exit(1);
                }
            };
        }
    }

    if let Some(ref thumbs) = thumbnails {
        if thumbs.is_empty() {
            thumbnails = None;
        }
    }

    (Some(install_dir), Some(app_id), thumbnails, game_name)
}

/** Returns a vector of `Game` instances parsed from .acf files in the specified directory.

 # Arguments

  `directory_path` - A reference to the directory containing .acf files.
  `thumb_path` - A reference to the directory containing game thumbnails.

 # Returns

 An `Option` containing a vector of `Game` instances if successful, or `None` if an error occurs.

 # Examples

 ```
 use std::path::Path;
 use oxi::config::steam::return_steamgames;
 let directory_path = Path::new("/path/to/directory");
 let thumb_path = Path::new("/path/to/thumbnails");
 let steam_games = return_steamgames(directory_path, thumb_path);
 ```
**/
pub fn return_steamgames(library: &Path, thumb_path: &Path) -> Option<Vec<Game>> {
    let mut steamgames: Vec<Game> = Vec::new();
    // Iterate over the entries in the directory
    match std::fs::read_dir(library) {
        Ok(entries) => {
            for entry in entries.flatten() {
                // Check if the entry is a file with a .acf extension
                match (entry.path().extension(), entry.file_type()) {
                    (Some(ext), Ok(file_type)) if ext == "acf" && file_type.is_file() => {
                        let acf_file = File::open(entry.path());
                        if let Ok(the_file) = acf_file {
                            let reader = BufReader::new(the_file);
                            let (install_path, app_id, thumbnail, game_name) =
                                parse_acf_files(library, thumb_path, reader);
                            if filter_banned_games(&game_name) {
                                continue;
                            }
                            // As long as they all exist, create the struct instance
                            if let (Some(app_id), Some(thumbnail), Some(game_name)) =
                                (app_id, thumbnail, game_name)
                            {
                                let game = Game {
                                    game_id: app_id,
                                    thumbnail,
                                    game_title: game_name,
                                    install_path,
                                    developer: None,
                                    publisher: None,
                                    save_path: None,
                                    saves: None,
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

fn read_file(path: PathBuf) -> String {
    match File::open(path) {
        Ok(opened_file) => {
            let mut reader = BufReader::new(opened_file);
            let mut content = String::new();
            reader
                .read_to_string(&mut content)
                .expect("Failed to read file");

            content
        }
        Err(e) => {
            eprintln!("Encountered error: {}.", e);
            panic!("Is steam installed?")
        }
    }
}

pub fn parse_app_id(input: &str) -> IResult<&str, u32> {
    let (input, _) = separated_pair(multispace0, tag("\"appid\""), multispace0)(input)?;
    let (input, app_id) = delimited(char('"'), nom::character::complete::u32, char('"'))(input)?;
    Ok((input, app_id))
}

pub fn parse_install_path<'a>(input: &'a str, library: &'a Path) -> IResult<&'a str, PathBuf> {
    let (input, _) = separated_pair(space0, tag("\"installdir\""), space1)(input)?;
    let (input, path) = map(
        delimited(char('"'), take_until("\""), char('"')),
        |path: &str| library.join(format!("common/{}", path)),
    )(input)?;
    Ok((input, path))
}

pub fn parse_name(input: &str) -> IResult<&str, String> {
    let (input, _) = separated_pair(space0, tag("\"name\""), space1)(input)?;
    let (input, game_name) = delimited(char('"'), take_until("\""), char('"'))(input)?;
    Ok((input, game_name.to_string()))
}
pub fn parse_path(input: &str) -> IResult<&str, PathBuf> {
    let (input, _) = separated_pair(space0, tag("\"path\""), space1)(input)?;
    let (input, path) = map(
        delimited(char('"'), take_until("\""), char('"')),
        |path: &str| PathBuf::from(format!("{}/steamapps/", path)),
    )(input)?;
    Ok((input, path))
}

/**
Parses a file to extract Steam library paths.

# Arguments

- `path` - The path to the file containing Steam library information.

# Returns

A vector containing the extracted Steam library paths.
*/
pub fn extract_steampath(file_path: PathBuf) -> Vec<PathBuf> {
    let content = read_file(file_path);
    let mut extracted_paths = Vec::new();

    for line in content.lines() {
        if line.contains("path") {
            let (_, path) = match parse_path(line) {
                Ok(path) => path,
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            };
            extracted_paths.push(path);
        }
    }

    extracted_paths
}

fn combine_steampaths(extracted_libraries: Vec<PathBuf>, thumb_path: PathBuf) -> Vec<Game> {
    let mut combined_steamgames: Vec<Game> = Vec::new();
    extracted_libraries.iter().for_each(|library| {
        if let Some(steamgames) = return_steamgames(library, &thumb_path) {
            combined_steamgames.extend(steamgames);
        } else {
            // Handle the case when return_steamgames returns None
            eprintln!("Failed to retrieve Steam games for {:?}", library);
        }
    });
    combined_steamgames
}

pub fn discover_games(verbose: bool) -> Vec<Game> {
    // TODO: Add data only for new games (aka not in config file) or upon force requested by user
    let home_dir = gen_home().expect("All OSes should have a home directory!??");
    let steam_lib: PathBuf = home_dir.join(".local/share/Steam/config/libraryfolders.vdf");
    let steam_thumb: PathBuf = home_dir.join(".local/share/Steam/appcache/librarycache");
    let steam_paths = extract_steampath(steam_lib.clone());
    let mut libraries = combine_steampaths(steam_paths, steam_thumb);
    libraries.iter_mut().for_each(|game| {
        game.find_compatdata();
    });
    println!(
        "\x1b[34mWe have found \x1b[31m{}\x1b[34m Steam games on your system!\x1b[0m ",
        libraries.len()
    );
    if verbose {
        libraries.sort_by(|a, b| a.game_title.cmp(&b.game_title));
        libraries.iter().for_each(|game| game.print_info());
    }
    libraries
}
