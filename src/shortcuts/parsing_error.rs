use std::error::Error;
use strum::Display;

#[derive(Debug, Display)]
pub enum ParsingError {
    WrongStartingByte,
    ShortcutsNotFound,
    ShortcutsEntriesDoNotAlign,
    InvalidMapType(u8),
    FailedToParseString,
    FailedToParseInteger,
    UnknownKey(String),
    CurrentShortcutEntryIsEmpty,
}

// impl Display for ParsingError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match self {
//             ParsingError::InvalidMapType(value) => {
//                 write!(f, "InvalidMapType at {}", value)
//             }
//             ParsingError::UnknownKey(value) => {
//                 write!(f, "UnknownKey {}", value)
//             }
//             _ => {
//                 write!(f, "{:?}", self)
//             }
//         }
//     }
// }

impl Error for ParsingError {}
