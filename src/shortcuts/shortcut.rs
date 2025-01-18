use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShortcutEntry {
    pub appid: u32,
    pub app_name: String,
    pub exe: String,
    pub start_dir: String,
    pub icon: String,
    pub shortcut_path: String,
    pub launch_options: String,
    pub is_hidden: u32,
    pub allow_desktop_config: u32,
    pub allow_overlay: u32,
    pub open_vr: u32,
    pub devkit: u32,
    pub devkit_game_id: String,
    pub devkit_override_app_id: u32,
    pub last_play_time: u32,
    pub flatpak_app_id: String,
    #[allow(dead_code)]
    pub tags: Vec<String>,
}
