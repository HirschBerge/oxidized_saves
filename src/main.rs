use std::path::PathBuf;
use oxi::settings::Settings;
use oxi::config::{game::Game, steam::{discover_steamgames, gen_home }, verify_conf, write_conf};


fn main() {
    // Get the user's home directory
    let home_dir = gen_home().expect("All OSes should have a home dir??"); 
    // Append the relative path to the user's home directory
    let settings_file = &home_dir.join(".config/oxi/oxi.json");
    let prog_settings: &Settings = &verify_conf::<Vec<Settings>>(settings_file.to_path_buf())[0];
    let game_conf_path = PathBuf::from(format!("{}/{}conf.json",home_dir.to_string_lossy(), prog_settings.game_conf_path.to_string_lossy()));
    let mut games: Vec<Game> = verify_conf(game_conf_path.clone());
    games[0].add_save(PathBuf::from("/mnt/storage/SteamLibrary/steamapps/compatdata/292030/"), &prog_settings.save_base_path);
    write_conf(games, &game_conf_path);
    discover_steamgames();
}

