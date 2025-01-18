use crate::shortcuts::shortcut::ShortcutEntry;
use crate::shortcuts::Shortcuts;
use crate::steam::get_user_id_fast;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Parser, Debug, Serialize, Deserialize, Clone)]
pub(crate) struct AddMinusGamesGameToSteam {
    #[arg(env)]
    pub game_path: PathBuf,
    #[arg(long, env, default_value = "true")]
    pub is_flatpak: bool,
    #[arg(long, env)]
    pub steam_id: Option<u32>,
}

#[derive(Parser, Debug, Serialize, Deserialize, Clone)]
pub(crate) struct AddFlatpak {
    pub flatpak_id: String,
}

#[derive(Debug, Subcommand, Serialize, Deserialize, strum::Display, Clone, Default)]
pub(crate) enum Actions {
    #[default]
    PrintShortcuts,
    PrintOnlyShortcuts,
    #[cfg(not(target_family = "windows"))]
    AddMinusGamesGameToSteam(AddMinusGamesGameToSteam),
    #[cfg(not(target_family = "windows"))]
    FixFlatpak,
    #[cfg(not(target_family = "windows"))]
    AddFlatpak(AddFlatpak),
}

#[derive(Parser, Debug, Serialize, Deserialize, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[command(subcommand)]
    pub action: Option<Actions>,
    #[arg(long, env)]
    pub steam_user_id: Option<String>,
    #[arg(long, env)]
    pub steam_shortcuts_file: Option<PathBuf>,
}

impl Config {
    pub(crate) fn get_action(&self) -> &Actions {
        self.action.as_ref().unwrap_or(&Actions::PrintShortcuts)
    }

    pub(crate) fn insert_shortcut(&self, shortcut_entry: ShortcutEntry) -> bool {
        let Some(shortcuts_file_path) = self.find_steam_shortcuts_file() else {
            return false;
        };

        let Some(mut shortcuts) = Shortcuts::from_path(&shortcuts_file_path) else {
            return false;
        };

        shortcuts.insert_shortcut(shortcut_entry);
        shortcuts.save_to(&shortcuts_file_path);

        true
    }

    pub(crate) fn get_grid_folder(&self) -> Option<PathBuf> {
        let steam_shortcuts_file_path = self.find_steam_shortcuts_file()?;
        let config_folder = steam_shortcuts_file_path.parent()?;
        Some(config_folder.join("grid"))
    }

    pub(crate) fn find_steam_shortcuts_file(&self) -> Option<PathBuf> {
        if let Some(steam_shortcuts_file) = &self.steam_shortcuts_file {
            return if steam_shortcuts_file.is_file() {
                Some(steam_shortcuts_file.clone())
            } else {
                None
            };
        }

        let mut rtn = dirs::home_dir()?;
        rtn.push(".steam/steam/userdata/");
        if let Some(steam_user_id) = &self.steam_user_id {
            rtn.push(steam_user_id);
        } else {
            let user_id = match get_user_id_fast() {
                Ok(value) => value,
                Err(_) => rtn
                    .read_dir()
                    .unwrap()
                    .find_map(|e| {
                        if let Ok(i) = e {
                            if i.file_name() != "0" && i.path().is_dir() {
                                return None;
                            }
                            return Some(i.file_name());
                        }
                        None
                    })?
                    .to_str()?
                    .to_string(),
            };

            rtn.push(user_id);
        }
        // if let Some(steam_user_id) = &self.steam_user_id {
        //     rtn.push(steam_user_id);
        // } else {
        //     rtn = rtn
        //         .read_dir()
        //         .ok()?
        //         .find(|e| {
        //             if let Ok(i) = e {
        //                 if i.file_name() != "0" && i.path().is_dir() {
        //                     return true;
        //                 }
        //             }
        //             false
        //         })?
        //         .ok()?
        //         .path();
        // }

        rtn.push("config/shortcuts.vdf");

        if rtn.is_file() {
            Some(rtn)
        } else {
            None
        }
    }
}
