use crate::TResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tokenizer<'a> {
    pub username: &'a str,
    pub sess_id: &'a str,
    pub sess_version: &'a str,
    pub nettype: &'a str,
    pub addrtype: &'a str,
    pub unicast_address: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a str) -> TResult<'a, Self> {
        use crate::parser_utils::*;
        use nom::{bytes::complete::tag, sequence::preceded};

        let (rem, line) = preceded(tag("o="), until_newline)(part)?;

        let (line_rem, username) = until_space(line)?;
        let (line_rem, sess_id) = until_space(line_rem)?;
        let (line_rem, sess_version) = until_space(line_rem)?;
        let (line_rem, nettype) = until_space(line_rem)?;
        let (unicast_address, addrtype) = until_space(line_rem)?;

        Ok((
            rem,
            Tokenizer {
                username,
                sess_id,
                sess_version,
                nettype,
                addrtype,
                unicast_address,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer() {
        let origin = concat!("o=jdoe 3724394400 3724394405 IN IP4 198.51.100.1\r\nsomething");

        assert_eq!(
            Tokenizer::tokenize(origin),
            Ok((
                "something",
                Tokenizer {
                    username: "jdoe",
                    sess_id: "3724394400",
                    sess_version: "3724394405",
                    nettype: "IN",
                    addrtype: "IP4",
                    unicast_address: "198.51.100.1"
                }
            )),
        );
    }
}
