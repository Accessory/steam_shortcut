use crate::config::Config;
use crate::shortcuts::Shortcuts;

pub(crate) fn print_shortcuts(config: &Config) {
    let Some(short_cuts_file_path) = config.find_steam_shortcuts_file() else {
        println!("Shortcuts file not found");
        return;
    };

    let shortcuts_file = Shortcuts::from_path(&short_cuts_file_path);
    match shortcuts_file {
        None => {
            println!("Shortcuts not found!");
        }
        Some(shortcuts) => {
            println!("Shortcuts at {}", short_cuts_file_path.display());
            println!("{}", shortcuts)
        }
    }
}

pub(crate) fn print_only_shortcuts(config: &Config) {
    let Some(short_cuts_file_path) = config.find_steam_shortcuts_file() else {
        println!("Shortcuts file not found");
        return;
    };

    let shortcuts_file = Shortcuts::from_path(&short_cuts_file_path);
    match shortcuts_file {
        None => {
            println!("{{}}");
        }
        Some(shortcuts) => {
            println!("{}", shortcuts)
        }
    }
}
