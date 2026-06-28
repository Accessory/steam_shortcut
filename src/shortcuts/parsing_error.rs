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
    UnknownStringKey(String),
    UnknownIntegerKey(String),
    CurrentShortcutEntryIsEmpty,
}

impl Error for ParsingError {}
