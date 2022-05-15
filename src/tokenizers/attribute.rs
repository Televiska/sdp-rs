use crate::TResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tokenizer<'a> {
    pub name: &'a str,
    pub value: Option<&'a str>,
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a str) -> TResult<'a, Self> {
        use crate::parser_utils::*;
        use nom::{branch::alt, bytes::complete::tag, combinator::rest, sequence::preceded};

        let (rem, name_with_value) = preceded(tag("a="), until_newline)(part)?;
        let (value, name) = alt((until_stopbreak_of(":"), rest))(name_with_value)?;
        let value = match value.is_empty() {
            true => None,
            false => Some(value),
        };

        Ok((rem, (name, value).into()))
    }
}

impl<'a> From<(&'a str, Option<&'a str>)> for Tokenizer<'a> {
    fn from((name, value): (&'a str, Option<&'a str>)) -> Self {
        Self { name, value }
    }
}

impl<'a> From<(&'a str, &'a str)> for Tokenizer<'a> {
    fn from((name, value): (&'a str, &'a str)) -> Self {
        Self {
            name,
            value: Some(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer1() {
        let attribute = concat!("a=recvonly\r\nsomething");

        assert_eq!(
            Ok((
                "something",
                Tokenizer {
                    name: "recvonly",
                    value: None,
                }
            )),
            Tokenizer::tokenize(attribute)
        );
    }

    #[test]
    fn tokenizer2() {
        let attribute = concat!("a=rtpmap:99 h263-1998/90000\r\nsomething");

        assert_eq!(
            Ok((
                "something",
                Tokenizer {
                    name: "rtpmap",
                    value: Some("99 h263-1998/90000"),
                }
            )),
            Tokenizer::tokenize(attribute)
        );
    }
}
