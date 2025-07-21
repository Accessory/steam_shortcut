use crate::config::{AddMinusGamesGameToSteam, Config};
use crate::shortcuts::shortcut::ShortcutEntry;
use crate::utils::{generate_app_id, try_download_grid_files};

pub(crate) fn add_minus_games_game_to_steam(
    config: &Config,
    sub_config: &AddMinusGamesGameToSteam,
) {
    if !sub_config.game_path.is_dir() {
        println!("Game path is not a directory");
        return;
    }

    let game_name = sub_config
        .game_path
        .iter()
        .next_back()
        .unwrap()
        .to_str()
        .unwrap();

    let shortcut =
        create_minus_games_shortcut_from_game_name(game_name, config, sub_config.steam_id);
    if !config.insert_shortcut(shortcut) {
        println!("Failed to add shortcut.")
    }
}

fn create_minus_games_shortcut_from_game_name(
    game_name: &str,
    config: &Config,
    steam_id: Option<u32>,
) -> ShortcutEntry {
    const EXE: &str = "/usr/bin/flatpak";

    let launch_options = format!(
        "run --branch=stable --arch=x86_64 --command=/app/bin/minus_games_gui io.github.accessory.minus_games_gui --mode Cli run-game-synced \"{game_name}\""
    );
    let appid = generate_app_id(EXE, &launch_options);

    let icon =
        if let Some((_, _, p, _)) = try_download_grid_files(game_name, config, appid, steam_id) {
            p.to_str().unwrap().to_string()
        } else {
            "".to_string()
        };

    ShortcutEntry {
        appid,
        app_name: game_name.to_string(),
        exe: EXE.to_string(),
        start_dir: "/usr/bin".to_string(),
        icon,
        shortcut_path: "".to_string(),
        launch_options,
        is_hidden: 0,
        allow_desktop_config: 1,
        allow_overlay: 1,
        open_vr: 0,
        devkit: 0,
        devkit_game_id: "".to_string(),
        devkit_override_app_id: 0,
        last_play_time: 0,
        flatpak_app_id: "".to_string(),
        tags: vec![],
    }
}
