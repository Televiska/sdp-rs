use crate::TResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tokenizer<'a> {
    pub nettype: &'a str,
    pub addrtype: &'a str,
    //TODO: Fix me
    pub connection_address: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a str) -> TResult<'a, Self> {
        use crate::parser_utils::*;
        use nom::{bytes::complete::tag, sequence::preceded};

        let (rem, nettype) = preceded(tag("c="), until_space)(part)?;
        let (rem, addrtype) = until_space(rem)?;
        let (rem, connection_address) = until_newline(rem)?;

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

impl<'a> From<(&'a str, &'a str, &'a str)> for Tokenizer<'a> {
    fn from((nettype, addrtype, connection_address): (&'a str, &'a str, &'a str)) -> Self {
        Self {
            nettype,
            addrtype,
            connection_address,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer() {
        let connection = concat!("c=IN IP4 198.51.100.1\r\nsomething");

        assert_eq!(
            Ok((
                "something",
                Tokenizer {
                    nettype: "IN",
                    addrtype: "IP4",
                    connection_address: "198.51.100.1"
                }
            )),
            Tokenizer::tokenize(connection)
        );
    }
}
