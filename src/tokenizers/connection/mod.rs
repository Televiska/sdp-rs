pub mod connection_address;

use crate::TResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tokenizer<'a> {
    pub nettype: &'a str,
    pub addrtype: &'a str,
    pub connection_address: connection_address::Tokenizer<'a>,
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a str) -> TResult<'a, Self> {
        use crate::parser_utils::*;
        use nom::{bytes::complete::tag, sequence::preceded};

        let (rem, nettype) = preceded(tag("c="), until_space)(part)?;
        let (rem, addrtype) = until_space(rem)?;
        let (rem, connection_address) = until_newline(rem)?;
        let (_, connection_address) = connection_address::Tokenizer::tokenize(connection_address)?;

        Ok((
            rem,
            Tokenizer {
                nettype,
                addrtype,
                connection_address,
            },
        ))
    }
}

impl<'a, T: Into<connection_address::Tokenizer<'a>>> From<(&'a str, &'a str, T)> for Tokenizer<'a> {
    fn from((nettype, addrtype, connection_address): (&'a str, &'a str, T)) -> Self {
        Self {
            nettype,
            addrtype,
            connection_address: connection_address.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connection_tokenizer1() {
        let connection = concat!("c=IN IP4 198.51.100.1\r\nsomething");

        assert_eq!(
            Tokenizer::tokenize(connection),
            Ok((
                "something",
                Tokenizer {
                    nettype: "IN",
                    addrtype: "IP4",
                    connection_address: connection_address::Tokenizer::from("198.51.100.1")
                }
            )),
        );
    }

    #[test]
    fn connection_tokenizer2() {
        let connection = concat!("c=IN IP4 233.252.0.1/127\r\nsomething");

        assert_eq!(
            Tokenizer::tokenize(connection),
            Ok((
                "something",
                Tokenizer {
                    nettype: "IN",
                    addrtype: "IP4",
                    connection_address: connection_address::Tokenizer {
                        base: "233.252.0.1",
                        ttl: Some("127"),
                        numaddr: None
                    }
                }
            )),
        );
    }

    #[test]
    fn connection_tokenizer3() {
        let connection = concat!("c=IN IP4 233.252.0.1/127/2\r\nsomething");

        assert_eq!(
            Tokenizer::tokenize(connection),
            Ok((
                "something",
                Tokenizer {
                    nettype: "IN",
                    addrtype: "IP4",
                    connection_address: connection_address::Tokenizer {
                        base: "233.252.0.1",
                        ttl: Some("127"),
                        numaddr: Some("2")
                    }
                }
            )),
        );
    }
}
