pub(crate) mod flatpak_creation_error;

use crate::desktop_file::DesktopFile;
use crate::flatpak::flatpak_creation_error::FlatPakCreationError;
use crate::shortcuts::shortcut::ShortcutEntry;
use crate::utils::{create_png_from_svg, fix_launch_options, generate_app_id};
use std::path::Path;
// pub(crate) fn add_flatpak_to_steam(flatpak_id: &str) {}

pub(crate) fn create_flatpak_shortcut(
    flatpak_id: &str,
) -> Result<ShortcutEntry, FlatPakCreationError> {
    let desktop_file_path = Path::new("/var/lib/flatpak/exports/share/applications/")
        .join(format!("{flatpak_id}.desktop"));

    if !desktop_file_path.is_file() {
        return Err(FlatPakCreationError::DesktopFileNotFound(format!(
            "Could not find a desktop file at {}",
            desktop_file_path.display()
        )));
    }

    let desktop_file: DesktopFile = DesktopFile::try_from_path(&desktop_file_path)
        .ok_or(FlatPakCreationError::CannotParseDesktopFile)?;
    const EXE: &str = "/usr/bin/flatpak";
    let appid = generate_app_id("/usr/bin/flatpak", flatpak_id);

    let icon = get_icon_path(&desktop_file.icon);

    let launch_options = fix_launch_options(desktop_file.to_exec());

    Ok(ShortcutEntry {
        appid,
        app_name: desktop_file.name,
        exe: EXE.to_string(),
        start_dir: "/usr/bin".to_string(),
        icon,
        shortcut_path: desktop_file_path.as_os_str().to_str().unwrap().to_string(),
        launch_options,
        is_hidden: 0,
        allow_desktop_config: 1,
        allow_overlay: 1,
        open_vr: 0,
        devkit: 0,
        devkit_game_id: "".to_string(),
        devkit_override_app_id: 0,
        last_play_time: 0,
        flatpak_app_id: flatpak_id.to_string(),
        tags: vec![],
    })
}

pub(crate) fn get_icon_path(icon: &str) -> String {
    let root = Path::new("/var/lib/flatpak/exports/share/icons/hicolor/");
    let scalable = root.join("scalable/apps/");

    let svg = scalable.join(format!("{icon}.svg"));
    if svg.is_file() {
        let sis = dirs::data_dir().unwrap().join("steam_icon_store");
        std::fs::create_dir_all(&sis).unwrap();
        let rtn_path = sis.join(format!("{icon}.png"));

        create_png_from_svg(&svg, &rtn_path);
        return rtn_path.as_os_str().to_str().unwrap().to_string();
    }

    let o512x512 = root.join("512x512/apps/");
    let png = o512x512.join(format!("{icon}.png"));
    if png.is_file() {
        return png.as_os_str().to_str().unwrap().to_string();
    }

    let o256x256 = root.join("256x256/apps/");
    let png = o256x256.join(format!("{icon}.png"));
    if png.is_file() {
        return png.as_os_str().to_str().unwrap().to_string();
    }

    let o128x128 = root.join("128x128/apps/");
    let png = o128x128.join(format!("{icon}.png"));
    if png.is_file() {
        return png.as_os_str().to_str().unwrap().to_string();
    }

    let o64x64 = root.join("64x64/apps/");
    let png = o64x64.join(format!("{icon}.png"));
    if png.is_file() {
        return png.as_os_str().to_str().unwrap().to_string();
    }

    String::new()
}
