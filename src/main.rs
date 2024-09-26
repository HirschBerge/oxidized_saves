use oxi::config::{
    game::Game,
    steam::{discover_steamgames, gen_home},
    verify_conf, write_conf,
};
use oxi::settings::Settings;
use std::path::PathBuf;

/// .
/// # Examples
///```
///    let games: Vec<Game> = verify_conf(game_conf_path.clone());
///    let search_item: &str = "WITCHER"
///    match search_games(&games, search_item.to_lowercase()) {
///        Ok(search) => {
///            println!("Title: {}\nSteam ID: {}\nSave Count: {}", search.game_title, search.steam_id, search.saves.len())
///        }
///        Err(err) => {
///            eprintln!("{}", err);
///            // Gracefully quit
///            std::process::exit(1);
///        }
///    }
///```
/// This function will return an error if the search result is not found
// TODO: Add searches by other metrics besides title, such as publisher or developer
#[allow(dead_code)]
fn search_games(games: &[Game], search: String) -> Result<&Game, &'static str> {
    match games
        .iter()
        .find(|&game| game.game_title.to_lowercase().contains(&search))
    {
        Some(game) => Ok(game),
        None => Err("Game not found"),
    }
}

fn main() {
    // Get the user's home directory
    let home_dir = gen_home().expect("All OSes should have a home dir??");
    // Append the relative path to the user's home directory
    let settings_file = &home_dir.join(".config/oxi/oxi.json");
    let prog_settings: &mut Settings =
        &mut verify_conf::<Vec<Settings>>(settings_file.to_path_buf())[0];
    let expanded_saves = PathBuf::from(format!(
        "{}/{}",
        &home_dir.display(),
        prog_settings.save_base_path.display()
    ));
    prog_settings.save_base_path = expanded_saves;
    let game_conf_path = PathBuf::from(format!(
        "{}/{}conf.json",
        home_dir.to_string_lossy(),
        prog_settings.game_conf_path.to_string_lossy()
    ));
    let games: Vec<Game> = verify_conf(game_conf_path.clone());

    // After modifications, write the `games` vector back to the configuration file
    write_conf(games, &game_conf_path);
    let discovered_games = discover_steamgames(false);
    // TEST: Shows layout of SteamGame. Here to debug when implementing merger
    write_conf(
        discovered_games,
        home_dir.join(".config/oxi/steam_games.json").as_path(),
    )
}
