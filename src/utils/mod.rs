use crate::config::Config;
use crate::shortcuts::parsing_error::ParsingError;
use crate::shortcuts::shortcut::ShortcutEntry;
use crate::steam::get_best_steam_app_id;
use crate::steamstatic::{download_600x900_2x, download_hero, download_logo};
use cosmic_text::{Attrs, Buffer, Color, FontSystem, Metrics, Shaping, SwashCache, Weight};
use image::imageops::FilterType;
use image::{GenericImageView, Pixel, RgbImage, RgbaImage};
use resvg::{tiny_skia, usvg};
use std::ops::AddAssign;
use std::path::{Path, PathBuf};

pub(crate) fn fix_launch_options(launch_options: String) -> String {
    let bytes = launch_options.as_bytes();
    let mut tokens: Vec<&str> = Vec::new();

    let mut pos = 0;
    let mut start = 0;
    let mut in_token = false;
    let mut skip_next = false;
    let mut in_quotation = false;

    while pos < bytes.len() {
        if skip_next {
            skip_next = false;
            continue;
        }

        let b = bytes[pos];

        if b == b'\\' {
            skip_next = true;
            continue;
        }

        if in_token {
            if in_quotation {
                if b == b'"' {
                    in_token = false;
                    in_quotation = false;
                    tokens.push(&launch_options[start..pos]);
                }
            } else if b == b' ' {
                in_token = false;
                tokens.push(&launch_options[start..pos]);
            }
        } else if b == b'"' {
            in_token = true;
            in_quotation = true;
            start = pos + 1;
        } else if b != b' ' {
            in_token = true;
            start = pos;
        }
        pos += 1
    }

    tokens.push(&launch_options[start..]);
    let mut rtn = String::new();

    let mut is_first = true;

    for token in tokens {
        if token.is_empty() || token == "%f" {
            continue;
        }
        if is_first {
            is_first = false;
        } else {
            rtn.push(' ');
        }
        rtn.push('"');
        rtn.push_str(token);
        rtn.push('"');
    }
    rtn
}

pub(crate) fn try_download_grid_files(
    game_name: &str,
    config: &Config,
    appid: u32,
    steam_id: Option<u32>,
) -> Option<(PathBuf, PathBuf, PathBuf, PathBuf)> {
    let Some(steam_id) = steam_id.or_else(|| get_best_steam_app_id(game_name)) else {
        println!("Could not get the steam id for {game_name}");
        return None;
    };

    let Some(grid_folder) = config.get_grid_folder() else {
        println!("Could not find the grid folder");
        return None;
    };
    if let Err(err) = download_hero(steam_id, appid, &grid_folder) {
        println!("Failed to download hero banner - {err}");
        return None;
    }

    let hero = grid_folder.join(format!("{appid}_hero.jpg"));
    let hero_cp = grid_folder.join(format!("{appid}.jpg"));
    std::fs::copy(&hero, &hero_cp).ok()?;

    if let Err(err) = download_600x900_2x(steam_id, appid, &grid_folder) {
        println!("Failed to download hero banner - {err}");
        return None;
    }
    if let Err(err) = download_logo(steam_id, appid, &grid_folder) {
        println!("Failed to download hero banner - {err}");
        return None;
    }

    Some((
        hero,
        hero_cp,
        grid_folder.join(format!("{appid}p.jpg")),
        grid_folder.join(format!("{appid}_logo.png")),
    ))
}

pub(crate) fn add_integer_to_shortcut(
    shortcut_entry_option: Option<&mut ShortcutEntry>,
    key: String,
    value: u32,
) -> Result<(), ParsingError> {
    if let Some(shortcut_entry) = shortcut_entry_option {
        match key.as_str() {
            "appid" => shortcut_entry.appid = value,
            "IsHidden" => shortcut_entry.is_hidden = value,
            "AllowDesktopConfig" => shortcut_entry.allow_desktop_config = value,
            "AllowOverlay" => shortcut_entry.allow_overlay = value,
            "OpenVR" => shortcut_entry.open_vr = value,
            "Devkit" => shortcut_entry.devkit = value,
            "DevkitOverrideAppID" => shortcut_entry.devkit_override_app_id = value,
            "LastPlayTime" => shortcut_entry.last_play_time = value,
            _ => return Err(ParsingError::UnknownKey(key)),
        }
    } else {
        return Err(ParsingError::CurrentShortcutEntryIsEmpty);
    }
    Ok(())
}

pub(crate) fn add_string_to_shortcut(
    shortcut_entry_option: Option<&mut ShortcutEntry>,
    key: String,
    value: String,
) -> Result<(), ParsingError> {
    if let Some(shortcut_entry) = shortcut_entry_option {
        match key.as_str() {
            "AppName" => shortcut_entry.app_name = value,
            "Exe" => shortcut_entry.exe = value,
            "StartDir" => shortcut_entry.start_dir = value,
            "icon" => shortcut_entry.icon = value,
            "ShortcutPath" => shortcut_entry.shortcut_path = value,
            "LaunchOptions" => shortcut_entry.launch_options = value,
            "DevkitGameID" => shortcut_entry.devkit_game_id = value,
            "FlatpakAppID" => shortcut_entry.flatpak_app_id = value,
            _ => return Err(ParsingError::UnknownKey(key)),
        }
    } else {
        return Err(ParsingError::CurrentShortcutEntryIsEmpty);
    }
    Ok(())
}

pub(crate) fn insert_entry_string_map(bytes: &mut Vec<u8>, key: &str) {
    bytes.push(0x00);
    insert_str_into_bytes(bytes, key);
    bytes.push(0x08);
}

pub(crate) fn insert_entry_string_integer(bytes: &mut Vec<u8>, key: &str, value: u32) {
    bytes.push(0x02);
    insert_str_into_bytes(bytes, key);
    insert_integer_into_bytes(bytes, value);
}

pub(crate) fn insert_entry_string_string(bytes: &mut Vec<u8>, key: &str, value: &str) {
    bytes.push(0x01);
    insert_str_into_bytes(bytes, key);
    insert_str_into_bytes(bytes, value);
}

pub(crate) fn insert_integer_into_bytes(bytes: &mut Vec<u8>, value: u32) {
    let integer_bytes: [u8; 4] = value.to_le_bytes();
    bytes.push(integer_bytes[0]);
    bytes.push(integer_bytes[1]);
    bytes.push(integer_bytes[2]);
    bytes.push(integer_bytes[3]);
}

pub(crate) fn insert_str_into_bytes(bytes: &mut Vec<u8>, value: &str) {
    for &i in value.as_bytes() {
        bytes.push(i);
    }
    bytes.push(0x00);
}

pub(crate) fn insert_string_into_bytes(bytes: &mut Vec<u8>, value: String) {
    for &i in value.as_bytes() {
        bytes.push(i);
    }
    bytes.push(0x00);
}

pub(crate) fn try_read_string(position: &mut usize, bytes: &[u8]) -> Result<String, ParsingError> {
    let mut rtn = String::new();

    while *position < bytes.len() {
        if bytes[*position] == 0x00 {
            position.add_assign(1);
            return Ok(rtn);
        }
        rtn.push(bytes[*position] as char);
        position.add_assign(1);
    }

    Err(ParsingError::FailedToParseString)
}

pub(crate) fn try_read_integer(position: &mut usize, bytes: &[u8]) -> Result<u32, ParsingError> {
    if *position + 4 >= bytes.len() {
        return Err(ParsingError::FailedToParseInteger);
    }
    let used_bytes: [u8; 4] = [
        bytes[*position],
        bytes[*position + 1],
        bytes[*position + 2],
        bytes[*position + 3],
    ];
    let rtn = u32::from_le_bytes(used_bytes);
    *position += 4;
    Ok(rtn)
}

pub(crate) fn generate_app_id(exe: &str, app_name: &str) -> u32 {
    const CRC_32: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
    let crc = CRC_32.checksum(format!("{exe}{app_name}").as_bytes());
    crc | 0x80000000
}

pub(crate) fn create_png_from_svg(from: &Path, to: &Path) {
    let tree = {
        let mut opt = usvg::Options {
            // Get file's absolute directory.
            resources_dir: std::fs::canonicalize(from)
                .ok()
                .and_then(|p| p.parent().map(|p| p.to_path_buf())),
            ..usvg::Options::default()
        };
        opt.fontdb_mut().load_system_fonts();

        let svg_data = std::fs::read(from).unwrap();
        usvg::Tree::from_data(&svg_data, &opt).unwrap()
    };

    let pixmap_size = tree.size().to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(256, 256).unwrap();
    let sx = 256.0f32 / pixmap_size.width() as f32;
    let sy = 256.0f32 / pixmap_size.height() as f32;
    resvg::render(
        &tree,
        tiny_skia::Transform::from_scale(sx, sy),
        &mut pixmap.as_mut(),
    );
    pixmap.save_png(to).unwrap();
}

pub(crate) fn draw_steam_logo_with_text(text: &str, to: &Path) {
    let mut font_system = FontSystem::new();
    font_system
        .db_mut()
        .load_font_data(include_bytes!("../../font/UbuntuSansMonoNerdFont-Medium.ttf").to_vec());
    // font_system
    //     .db_mut()
    //     .load_font_data(include_bytes!("../../font/UbuntuSansMonoNerdFontMono-Regular.ttf").to_vec());

    // println!("Fonts");
    let mut font_name_option = None;
    let mut weight = Weight::NORMAL;
    for face in font_system.db().faces() {
    //     // println!("Font: {}", &face.families.first().unwrap().0);
        if face.families.first().unwrap().0 == "Motiva Sans" {
            font_name_option = Some(face.families.first().unwrap().0.clone());
            break;
        }
    }

    let font_name = font_name_option.unwrap_or_else(|| {
        weight = Weight::MEDIUM;
        font_system
            .db()
            .faces()
            .last()
            .unwrap()
            .families
            .first()
            .unwrap()
            .0
            .clone()
    });

    println!("Font Name: {font_name}");

    let mut swash_cache = SwashCache::new();

    let font_size: f32 = 110.0 - ((text.len() / 10) * 20) as f32;
    let line_height: f32 = font_size;
    let metrics = Metrics::new(font_size, line_height);
    let mut buffer = Buffer::new(&mut font_system, metrics);
    let mut buffer = buffer.borrow_with(&mut font_system);
    let width = 640.0;
    let height = 360.0;
    buffer.set_size(Some(width), Some(height));

    let attrs = Attrs::new()
        .family(cosmic_text::Family::Name(&font_name))
        .weight(weight);
    // let attrs = Attrs::new().family(cosmic_text::Family::Name("AurulentSansM Nerd Font Mono"));

    buffer.set_rich_text(
        [(text, attrs.clone())],
        &attrs,
        Shaping::Advanced,
        Some(cosmic_text::Align::Center),
    );
    buffer.shape_until_scroll(true);
    const TEXT_COLOR: Color = Color::rgb(0xFF, 0xFF, 0xFF);
    let mut canvas = vec![vec![None; width as usize]; height as usize];

    let mut last_y = 0;

    buffer.draw(&mut swash_cache, TEXT_COLOR, |x, y, w, h, color| {
        let a = color.a();
        if a == 0 || x < 0 || x >= width as i32 || y < 0 || y >= height as i32 || w != 1 || h != 1 {
            return;
        }

        // canvas[y as usize][x as usize] = Some((0, 0, 0, 0xFF));
        canvas[y as usize][x as usize] = Some((0xFF, 0xFF, 0xFF, 0xFF));

        last_y = last_y.max(y);
    });

    let half_last_y = last_y / 2;
    let y_start = 360 / 2 - half_last_y - 10;

    let rows = canvas.len();
    let columns = canvas.first().unwrap().len();

    let mut image = RgbaImage::new(columns as u32, rows as u32);

    for (y, row) in canvas.iter().enumerate() {
        let used_y = (y as i32).checked_add(y_start).unwrap_or(0);
        if used_y >= height as i32 {
            break;
        }
        for (x, pixel) in row.iter().enumerate() {
            let current_pixel = image.get_pixel_mut(x as u32, used_y as u32);
            let color = pixel.unwrap_or((0, 0, 0, 0));
            current_pixel.0 = [color.0, color.1, color.2, color.3];
        }
    }

    image.save(to).unwrap();
}

pub(crate) fn create_grid_for_shortcut(shortcut: &ShortcutEntry, grid_path: &Path) {
    let icon = image::open(&shortcut.icon).unwrap();
    let mut img = icon.resize(400, 400, FilterType::Nearest);
    let x = (600 - img.width()) / 2;
    let y = (900 - img.height()) / 2;

    let tl = img.get_pixel(0, 0).to_rgb();
    let tr = img.get_pixel(img.width() - 1, 0).to_rgb();
    let bl = img.get_pixel(0, img.height() - 1).to_rgb();
    let br = img.get_pixel(img.width() - 1, img.height() - 1).to_rgb();

    let mut final_image = RgbImage::new(2, 2);
    final_image.put_pixel(0, 0, tl);
    final_image.put_pixel(1, 0, tr);
    final_image.put_pixel(0, 1, bl);
    final_image.put_pixel(1, 1, br);

    final_image = image::imageops::resize(&final_image, 600, 900, FilterType::Triangle);
    image::imageops::overlay(&mut final_image, &img.to_rgb8(), x as i64, y as i64);

    let p_image_path = grid_path.join(format!("{}p.jpg", shortcut.appid));

    let parent = p_image_path.parent().unwrap();
    std::fs::create_dir(parent).ok();

    final_image
        .save_with_format(p_image_path, image::ImageFormat::Jpeg)
        .unwrap();

    let mut blur = icon.resize_to_fill(1920, 620, FilterType::Nearest);
    blur = blur.fast_blur(100.0);
    img = icon.resize(1920, 620, FilterType::Gaussian);
    let x = (1920 - img.width()) / 2;
    let y = (620 - img.height()) / 2;
    image::imageops::overlay(&mut blur, &img, x as i64, y as i64);
    let rgb = blur.to_rgb8();
    let base_image_path = grid_path.join(format!("{}.jpg", shortcut.appid));
    rgb.save_with_format(base_image_path, image::ImageFormat::Jpeg)
        .unwrap();
    let hero_image_path = grid_path.join(format!("{}_hero.jpg", shortcut.appid));
    rgb.save_with_format(hero_image_path, image::ImageFormat::Jpeg)
        .unwrap();

    // Logo
    // let mut logo = image::RgbaImage::new(640, 360);
    // let font = FontRef::try_from_slice(include_bytes!(
    //     "../../font/AurulentSansMNerdFontMono-Regular.otf"
    // ))
    // .unwrap();
    // const LOGO_DEFAULT_HEIGHT: f32 = 15.0;
    // let scale = PxScale {
    //     x: LOGO_DEFAULT_HEIGHT,
    //     y: LOGO_DEFAULT_HEIGHT * 1.5,
    // };
    // let (width, height) = text_size(scale, &font, &shortcut.app_name);

    // const WANT_TO_SCALE_TO_WIDTH: f32 = 640.0 * 0.7;
    // let width_scaling_factor = WANT_TO_SCALE_TO_WIDTH / width as f32;

    // const WANT_TO_SCALE_TO_HEIGHT: f32 = 360.0 * 0.7;
    // let height_scaling_factor = WANT_TO_SCALE_TO_HEIGHT / height as f32;

    // let finale_scale = PxScale {
    //     x: LOGO_DEFAULT_HEIGHT * width_scaling_factor,
    //     y: LOGO_DEFAULT_HEIGHT * height_scaling_factor,
    // };

    // let (final_width, final_height) = text_size(finale_scale, &font, &shortcut.app_name);

    // let half_width = (final_width / 2) as i32;
    // let half_height = (final_height / 2) as i32;

    // let start_x = 640 / 2 - half_width;
    // let start_y = 360 / 2 - half_height - 100;

    // imageproc::drawing::draw_text_mut(
    //     &mut logo,
    //     Rgba([255u8, 255u8, 255u8, 255u8]),
    //     start_x,
    //     start_y,
    //     finale_scale,
    //     &font,
    //     &shortcut.app_name,
    // );
    // println!("Text size: {final_width}x{final_height}");

    let logo_path = grid_path.join(format!("{}_logo.png", shortcut.appid));
    draw_steam_logo_with_text(&shortcut.app_name, &logo_path);
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::utils::{draw_steam_logo_with_text, fix_launch_options};

    #[test]
    fn test_launch_options_fixer() {
        let launch_options = "run --branch=stable --arch=x86_64 --command=boxy-svg --file-forwarding com.boxy_svg.BoxySVG @@ %f @@";
        let result = fix_launch_options(launch_options.to_string());
        println!("{}", &result);
        assert_eq!(
            r#""run" "--branch=stable" "--arch=x86_64" "--command=boxy-svg" "--file-forwarding" "com.boxy_svg.BoxySVG" "@@" "@@""#,
            result.as_str()
        )
    }

    #[test]
    fn draw_text_with_cosmic() {
        draw_steam_logo_with_text("Heroic Games Launcher", &Path::new("target/result.png"));
        draw_steam_logo_with_text("PG Admin 4", &Path::new("target/result2.png"));
        draw_steam_logo_with_text("Minus Games", &Path::new("target/result3.png"));
    }
}
