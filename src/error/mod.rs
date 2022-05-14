mod tokenizer_error;

pub use tokenizer_error::TokenizerError;

use std::{error::Error as StdError, fmt};

/// The `Error` enum indicates that something went wrong
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Error {
    ParseError(String),
    TokenizeError(String),
    Incomplete,
}

impl Error {
    pub fn tokenizer<'a, S, T>(tuple: (S, T)) -> Self
    where
        S: std::fmt::Display,
        T: std::fmt::Display,
    {
        Self::TokenizeError(format!("failed to tokenize {}: {}", tuple.0, tuple.1))
    }
}

impl From<TokenizerError> for Error {
    fn from(from: TokenizerError) -> Self {
        Self::TokenizeError(from.context)
    }
}

impl From<nom::Err<TokenizerError>> for Error {
    fn from(from: nom::Err<TokenizerError>) -> Self {
        match from {
            nom::Err::Incomplete(_) => Self::Incomplete,
            nom::Err::Error(e) => e.into(),
            nom::Err::Failure(e) => e.into(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::TokenizeError(inner) => write!(f, "Tokenizer error: {}", inner),
            Self::ParseError(inner) => write!(f, "sdp error: could not parse part: {}", inner),
            Self::Incomplete => write!(f, "sdp error: incomplete input"),
        }
    }
}

impl StdError for Error {}

impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Self {
        Self::ParseError(error.to_string())
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(error: std::net::AddrParseError) -> Self {
        Self::ParseError(error.to_string())
    }
}