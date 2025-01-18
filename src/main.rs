use crate::actions::flatpak::{add_flatpak, fix_flatpak};
use crate::actions::minus_games::add_minus_games_game_to_steam;
use crate::actions::print_shortcuts::{print_only_shortcuts, print_shortcuts};
use crate::config::{Actions, Config};
use clap::Parser;

mod actions;
mod config;
mod desktop_file;
mod flatpak;
mod pc_gaming_wiki;
mod shortcuts;
mod steam;
mod steamstatic;
mod utils;

fn main() {
    dotenvy::dotenv().ok();
    let config = Config::parse();

    match &config.get_action() {
        Actions::PrintShortcuts => {
            print_shortcuts(&config);
        }
        Actions::PrintOnlyShortcuts => {
            print_only_shortcuts(&config);
        }
        #[cfg(not(target_family = "windows"))]
        Actions::AddMinusGamesGameToSteam(sub_config) => {
            add_minus_games_game_to_steam(&config, sub_config);
        }
        #[cfg(not(target_family = "windows"))]
        Actions::AddFlatpak(add_flatpak_config) => add_flatpak(&config, add_flatpak_config),
        #[cfg(not(target_family = "windows"))]
        Actions::FixFlatpak => {
            fix_flatpak(config);
        }
    }
}
