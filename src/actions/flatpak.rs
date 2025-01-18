use crate::config::{AddFlatpak, Config};
use crate::flatpak::create_flatpak_shortcut;
use crate::shortcuts::Shortcuts;
use crate::utils::create_grid_for_shortcut;

pub(crate) fn fix_flatpak(config: Config) {
    let Some(shortcuts_file_path) = config.find_steam_shortcuts_file() else {
        println!("Failed to find the shortcuts");
        return;
    };
    let Ok(bytes) = std::fs::read(&shortcuts_file_path) else {
        println!("Could not read the shortcuts file");
        return;
    };
    let Ok(mut shortcuts) = Shortcuts::try_from_bytes(&bytes) else {
        println!(
            "Failed to parse the shortcuts file at {}",
            shortcuts_file_path.display()
        );
        return;
    };
    if shortcuts.fix_flatpaks(&shortcuts_file_path.parent().unwrap().join("grid")) {
        shortcuts.save_to(&shortcuts_file_path);
    }
}

pub(crate) fn add_flatpak(config: &Config, add_flatpak_config: &AddFlatpak) {
    let shortcut = match create_flatpak_shortcut(&add_flatpak_config.flatpak_id) {
        Ok(value) => value,
        Err(err) => {
            println!(
                "Failed to create a shortcut for flatpak {} - Err {err:?}",
                add_flatpak_config.flatpak_id
            );
            return;
        }
    };

    if let Some(grid_path) = config.get_grid_folder() {
        create_grid_for_shortcut(&shortcut, &grid_path);
    }

    if !config.insert_shortcut(shortcut) {
        println!("Failed to insert shortcut into the shortcuts file.")
    }
}
