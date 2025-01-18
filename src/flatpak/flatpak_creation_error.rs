#[derive(Debug)]
pub enum FlatPakCreationError {
    #[allow(dead_code)]
    DesktopFileNotFound(String),
    CannotParseDesktopFile,
}
