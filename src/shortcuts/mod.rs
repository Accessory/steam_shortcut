pub(crate) mod parsing_error;
pub(crate) mod shortcut;

use crate::desktop_file::DesktopFile;
use crate::flatpak::get_icon_path;
use crate::shortcuts::parsing_error::ParsingError;
use crate::shortcuts::shortcut::ShortcutEntry;
use crate::utils::{
    add_integer_to_shortcut, add_string_to_shortcut, create_grid_for_shortcut,
    insert_entry_string_integer, insert_entry_string_map, insert_entry_string_string,
    insert_str_into_bytes, insert_string_into_bytes, try_read_integer, try_read_string,
};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::AddAssign;
use std::path::Path;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Shortcuts {
    pub shortcuts: Vec<ShortcutEntry>,
}

impl Display for Shortcuts {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}

impl Shortcuts {
    pub(crate) fn insert_shortcut(&mut self, shortcut_entry: ShortcutEntry) {
        self.shortcuts.retain(|i| i.appid != shortcut_entry.appid);
        self.shortcuts.push(shortcut_entry);
    }

    pub(crate) fn from_path(path: &Path) -> Option<Self> {
        let bytes = std::fs::read(path).ok()?;
        Self::try_from_bytes(&bytes).ok()
    }

    pub(crate) fn save_to(&self, shortcuts_file_path: &Path) {
        let bytes = self.to_bytes();
        std::fs::write(shortcuts_file_path, bytes).unwrap()
    }

    pub(crate) fn fix_flatpaks(&mut self, grid_path: &Path, force: bool) -> bool {
        let mut has_changes = false;
        for shortcut in self.shortcuts.iter_mut() {
            if !force && (!shortcut.icon.is_empty() || !shortcut.exe.contains("flatpak")) {
                continue;
            }

            // println!("Change Shortcut: {}", &shortcut.app_name);

            if !shortcut.shortcut_path.is_empty() {
                if let Some(desktop_file) =
                    DesktopFile::try_from_path(Path::new(&shortcut.shortcut_path))
                {
                    let path = get_icon_path(&desktop_file.icon);
                    if !path.is_empty() {
                        shortcut.icon = path;
                        create_grid_for_shortcut(shortcut, grid_path);
                        shortcut.flatpak_app_id = desktop_file.icon;
                        has_changes = true;
                        println!("Changed Shortcut: {}", &shortcut.app_name);
                    } else {
                        println!("Icon path is empty");
                    }
                } else {
                    println!("could not parse the desktop file");
                }
            } else {
                println!("shortcut_path is empty");
            }
        }
        has_changes
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut rtn = Vec::new();

        rtn.push(0x00);
        insert_str_into_bytes(&mut rtn, "shortcuts");

        for (i, shortcut) in self.shortcuts.iter().enumerate() {
            rtn.push(0x00);
            insert_string_into_bytes(&mut rtn, i.to_string());
            insert_entry_string_integer(&mut rtn, "appid", shortcut.appid);
            insert_entry_string_string(&mut rtn, "AppName", shortcut.app_name.as_str());
            insert_entry_string_string(&mut rtn, "Exe", shortcut.exe.as_str());
            insert_entry_string_string(&mut rtn, "StartDir", shortcut.start_dir.as_str());
            insert_entry_string_string(&mut rtn, "icon", shortcut.icon.as_str());
            insert_entry_string_string(&mut rtn, "ShortcutPath", shortcut.shortcut_path.as_str());
            insert_entry_string_string(&mut rtn, "LaunchOptions", shortcut.launch_options.as_str());
            insert_entry_string_integer(&mut rtn, "IsHidden", shortcut.is_hidden);
            insert_entry_string_integer(
                &mut rtn,
                "AllowDesktopConfig",
                shortcut.allow_desktop_config,
            );
            insert_entry_string_integer(&mut rtn, "AllowOverlay", shortcut.allow_overlay);
            insert_entry_string_integer(&mut rtn, "OpenVR", shortcut.open_vr);
            insert_entry_string_integer(&mut rtn, "Devkit", shortcut.devkit);
            insert_entry_string_string(&mut rtn, "DevkitGameID", shortcut.devkit_game_id.as_str());
            insert_entry_string_integer(
                &mut rtn,
                "DevkitOverrideAppID",
                shortcut.devkit_override_app_id,
            );
            insert_entry_string_integer(&mut rtn, "LastPlayTime", shortcut.last_play_time);
            insert_entry_string_string(&mut rtn, "FlatpakAppID", shortcut.flatpak_app_id.as_str());
            insert_entry_string_map(&mut rtn, "tags");
            rtn.push(0x08);
        }
        rtn.push(0x08);
        rtn.push(0x08);
        rtn
    }

    pub fn try_from_bytes(bytes: &[u8]) -> Result<Shortcuts, ParsingError> {
        let mut parsing_state = ParsingState::Start;
        let mut rtn = Shortcuts::default();
        let mut current_shortcut = None;

        let mut position = 0;
        let mut shortcuts_position = 0;

        while position < bytes.len() {
            match parsing_state {
                ParsingState::Start => {
                    if bytes[position] != 0x00 {
                        return Err(ParsingError::WrongStartingByte);
                    }
                    parsing_state = ParsingState::Shortcuts;
                    position = 1;
                }
                ParsingState::Shortcuts => {
                    let substring = &bytes[position..position + SHORTCUTS_STRING.len()];
                    if substring != SHORTCUTS_STRING.as_bytes() {
                        return Err(ParsingError::ShortcutsNotFound);
                    }
                    position = 3 + SHORTCUTS_STRING.len();
                    parsing_state = ParsingState::Shortcut;
                }
                ParsingState::Shortcut => {
                    if bytes[position] == 0x08 {
                        position.add_assign(1);
                        continue;
                    }

                    let result = try_read_string(&mut position, bytes)?;
                    if result != shortcuts_position.to_string() {
                        return Err(ParsingError::ShortcutsEntriesDoNotAlign);
                    }
                    shortcuts_position += 1;
                    parsing_state = ParsingState::Entry;
                    current_shortcut = Some(ShortcutEntry::default())
                }

                ParsingState::Entry => {
                    match bytes[position] {
                        0x00 => {
                            parsing_state = ParsingState::StringMap;
                        }
                        0x01 => {
                            parsing_state = ParsingState::StringString;
                        }
                        0x02 => {
                            parsing_state = ParsingState::StringInteger;
                        }
                        0x08 => {
                            let shortcut_entry = current_shortcut
                                .take()
                                .ok_or(ParsingError::CurrentShortcutEntryIsEmpty)?;

                            rtn.shortcuts.push(shortcut_entry);
                            position.add_assign(1);
                            parsing_state = ParsingState::Shortcut;
                        }
                        _ => return Err(ParsingError::InvalidMapType(bytes[position])),
                    }
                    position += 1;
                }
                ParsingState::StringString => {
                    let key = try_read_string(&mut position, bytes)?;
                    let value = try_read_string(&mut position, bytes)?;
                    add_string_to_shortcut(current_shortcut.as_mut(), key, value)?;
                    parsing_state = ParsingState::Entry;
                }
                ParsingState::StringInteger => {
                    let key = try_read_string(&mut position, bytes)?;
                    let value = try_read_integer(&mut position, bytes)?;
                    add_integer_to_shortcut(current_shortcut.as_mut(), key, value)?;
                    parsing_state = ParsingState::Entry;
                }
                ParsingState::StringMap => {
                    while position < bytes.len() {
                        if bytes[position] == 0x08 {
                            position.add_assign(1);
                            break;
                        }
                        position.add_assign(1);
                    }
                    parsing_state = ParsingState::Entry;
                }
            }
        }

        Ok(rtn)
    }
}

enum ParsingState {
    Start,
    Shortcuts,
    Shortcut,
    Entry,
    StringString,
    StringInteger,
    StringMap,
}

const SHORTCUTS_STRING: &str = "shortcuts";
