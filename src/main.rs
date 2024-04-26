use oxi::config::{
    game::Game,
    steam::{discover_steamgames, gen_home},
    verify_conf, write_conf,
};
use oxi::settings::Settings;
use std::path::PathBuf;

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
    let mut games: Vec<Game> = verify_conf(game_conf_path.clone());
    // Modify the first game in the `games` vector
    if let Some(tw3) = games.first_mut() {
        tw3.add_save(tw3.save_path.clone(), &prog_settings.save_base_path);
        // Access the last save in the first game's `saves` vector
        if let Some(new_save) = tw3.saves.last_mut() {
            // TEST: commenting out the actual backup/restore until i can write tests
            // so i don't back bacon ruining my storage space.
            new_save.backup();
            new_save.restore();
        } else {
            eprintln!("No valid save found for backup.");
        }
    }

    // After modifications, write the `games` vector back to the configuration file
    write_conf(games, &game_conf_path);
    discover_steamgames();
}
