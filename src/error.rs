use std::fmt;

/// An application error that is unrecoverable in the context of a single request, such as an I/O
/// error or a programming error.
#[derive(Debug)]
pub enum Error {
    R2D2Error(r2d2::Error),
    RusqliteError(rusqlite::Error),
    IntentParserError(::failure::Error),
    UnknownIntent(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::R2D2Error(error) => write!(f, "Connection pool error: {}", error),
            Error::RusqliteError(error) => write!(f, "Database error: {}", error),
            Error::IntentParserError(error) => write!(f, "Intent parser error: {}", error),
            Error::UnknownIntent(intent_name) => write!(f, "Unknown intent: {}", intent_name),
        }
    }
}
