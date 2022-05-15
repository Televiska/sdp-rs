use crate::TResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tokenizer<'a> {
    pub base: &'a str,
    pub ttl: Option<&'a str>,
    pub numaddr: Option<&'a str>,
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a str) -> TResult<'a, Self> {
        use nom::{
            branch::alt,
            bytes::complete::{tag, take_until},
            combinator::rest,
            sequence::terminated,
        };

        let (rem, base) = alt((terminated(take_until("/"), tag("/")), rest))(part)?;
        let (rem, ttl) = alt((terminated(take_until("/"), tag("/")), rest))(rem)?;
        let (_, numaddr) = alt((terminated(take_until("/"), tag("/")), rest))(rem)?;
        let ttl = match ttl.is_empty() {
            true => None,
            false => Some(ttl),
        };
        let numaddr = match numaddr.is_empty() {
            true => None,
            false => Some(numaddr),
        };

        Ok(("", (base, ttl, numaddr).into()))
    }
}

impl<'a> From<(&'a str, Option<&'a str>, Option<&'a str>)> for Tokenizer<'a> {
    fn from((base, ttl, numaddr): (&'a str, Option<&'a str>, Option<&'a str>)) -> Self {
        Self { base, ttl, numaddr }
    }
}

impl<'a> From<(&'a str, &'a str, &'a str)> for Tokenizer<'a> {
    fn from((base, ttl, numaddr): (&'a str, &'a str, &'a str)) -> Self {
        Self {
            base,
            ttl: Some(ttl),
            numaddr: Some(numaddr),
        }
    }
}

impl<'a> From<&'a str> for Tokenizer<'a> {
    fn from(base: &'a str) -> Self {
        Self {
            base,
            ttl: None,
            numaddr: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connection_address_tokenizer1() {
        let connection = concat!("198.51.100.1");

        assert_eq!(
            Tokenizer::tokenize(connection),
            Ok((
                "",
                Tokenizer {
                    base: "198.51.100.1",
                    ttl: None,
                    numaddr: None
                }
            )),
        );
    }

    #[test]
    fn connection_address_tokenizer2() {
        let connection = concat!("233.252.0.1/127");

        assert_eq!(
            Tokenizer::tokenize(connection),
            Ok((
                "",
                Tokenizer {
                    base: "233.252.0.1",
                    ttl: Some("127"),
                    numaddr: None
                }
            )),
        );
    }

    #[test]
    fn connection_address_tokenizer3() {
        let connection = concat!("233.252.0.1/127/2");

        assert_eq!(
            Tokenizer::tokenize(connection),
            Ok((
                "",
                Tokenizer {
                    base: "233.252.0.1",
                    ttl: Some("127"),
                    numaddr: Some("2")
                }
            )),
        );
    }
}
