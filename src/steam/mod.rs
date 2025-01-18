use crate::steam::app_list_result::AppListResult;
use crate::steam::login_users::LoginUsers;
use anyhow::anyhow;
use textdistance::str::sift4_simple;

mod app_list_result;
pub mod login_users;

pub(crate) fn get_user_id_fast() -> anyhow::Result<String> {
    let login_users_path = dirs::home_dir()
        .ok_or(anyhow!("Could not find home dir"))?
        .join(".steam/steam/config/loginusers.vdf");

    LoginUsers::get_first_user_id(&login_users_path).ok_or(anyhow!("Could not find first user_id"))
}

pub(crate) fn get_best_steam_app_id(game_name: &str) -> Option<u32> {
    let mut data = ureq::get("https://api.steampowered.com/ISteamApps/GetAppList/v2/?format=json")
        .call()
        .expect("Failed to get the app list.");
    let app_list_result: AppListResult = data
        .body_mut()
        .read_json()
        .expect("Failed to extract app_list_result json");
    let game_name_lowercase = game_name.to_ascii_lowercase();
    let result = app_list_result
        .app_list
        .apps
        .iter()
        .min_by_key(|a| sift4_simple(&game_name_lowercase, &a.name.to_ascii_lowercase()))?;

    println!(
        "Found Steam AppId: {} - Name: {}",
        result.app_id, &result.name
    );
    Some(result.app_id as u32)
}
