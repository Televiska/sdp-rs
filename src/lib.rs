mod error;
pub mod tokenizers;

pub use error::{Error, TokenizerError};

pub(crate) type TResult<'a, T> = Result<(&'a str, T), nom::Err<TokenizerError>>;
pub(crate) type SResult<'a> = Result<(&'a str, &'a str), nom::Err<TokenizerError>>;

pub struct SessionDescription {
    pub version: String,
    pub origin: String,
    pub name: String,
    pub information: Option<String>,
    pub uri: Option<String>,
    pub emails: Vec<String>,
    pub phones: Vec<String>,
    pub connection: Option<String>,
    pub bandwidths: Vec<String>,
    pub time_description: String,
    pub key: Option<String>,
    pub attributes: Vec<String>,
    pub medias: Vec<String>,
}

//TODO: add tests
pub(crate) mod parser_utils {
    use crate::SResult;

    pub fn until_stopbreak_of<'a>(stopbreak: &'a str) -> impl FnMut(&'a str) -> SResult<'a> {
        use nom::{
            bytes::complete::{tag, take_until},
            sequence::terminated,
        };

        terminated(take_until(stopbreak), tag(stopbreak))
    }

    pub fn until_space(part: &str) -> SResult {
        use nom::{
            bytes::complete::{tag, take_until},
            sequence::terminated,
        };

        terminated(take_until(" "), tag(" "))(part)
    }

    pub fn until_newline(part: &str) -> SResult {
        use nom::branch::alt;

        alt((until_crlf, until_cr, until_lf))(part)
    }

    fn until_crlf(part: &str) -> SResult {
        use nom::{
            bytes::complete::{tag, take_until},
            sequence::terminated,
        };

        terminated(take_until("\r\n"), tag("\r\n"))(part)
    }

    fn until_cr(part: &str) -> SResult {
        use nom::{
            bytes::complete::{tag, take_until},
            sequence::terminated,
        };

        terminated(take_until("\r"), tag("\r"))(part)
    }

    fn until_lf(part: &str) -> SResult {
        use nom::{
            bytes::complete::{tag, take_until},
            sequence::terminated,
        };

        terminated(take_until("\n"), tag("\n"))(part)
    }
}
