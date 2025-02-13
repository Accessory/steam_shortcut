use std::error::Error;
use std::io::BufWriter;
use std::path::Path;

const STEAM_DB_BASE_URL: &str = "https://cdn.cloudflare.steamstatic.com/steam/apps";

pub(crate) fn download_hero(steam_id: u32, appid: u32, to: &Path) -> Result<(), Box<dyn Error>> {
    let url = format!("{STEAM_DB_BASE_URL}/{steam_id}/library_hero.jpg");
    let mut reader = ureq::get(&url).call()?.into_body().into_reader();

    let final_to = to.join(format!("{appid}_hero.jpg"));
    let file = std::fs::File::create(final_to)?;
    let mut writer = BufWriter::new(file);

    std::io::copy(&mut reader, &mut writer)?;

    Ok(())
}
pub(crate) fn download_600x900_2x(
    steam_id: u32,
    appid: u32,
    to: &Path,
) -> Result<(), Box<dyn Error>> {
    let url = format!("{STEAM_DB_BASE_URL}/{steam_id}/library_600x900_2x.jpg");
    let mut reader = ureq::get(&url).call()?.into_body().into_reader();

    let final_to = to.join(format!("{appid}p.jpg"));
    let file = std::fs::File::create(final_to)?;
    let mut writer = BufWriter::new(file);

    std::io::copy(&mut reader, &mut writer)?;

    Ok(())
}
pub(crate) fn download_logo(steam_id: u32, appid: u32, to: &Path) -> Result<(), Box<dyn Error>> {
    let url = format!("{STEAM_DB_BASE_URL}/{steam_id}/logo.png");
    let mut reader = ureq::get(&url).call()?.into_body().into_reader();

    let final_to = to.join(format!("{appid}_logo.png"));
    let file = std::fs::File::create(final_to)?;
    let mut writer = BufWriter::new(file);

    std::io::copy(&mut reader, &mut writer)?;

    Ok(())
}
