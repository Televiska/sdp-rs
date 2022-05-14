mod error;

pub use error::{Error, TokenizerError};

pub(crate) type TResult<'a, T> = Result<(&'a str, T), nom::Err<TokenizerError>>;
pub(crate) type SResult<'a> = Result<(&'a str, &'a str), nom::Err<TokenizerError>>;
