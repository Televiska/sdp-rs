use crate::TResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tokenizer<'a, const C: char> {
    pub key: &'a str,
    pub value: Option<&'a str>,
}

impl<'a, const C: char> Tokenizer<'a, C> {
    pub fn tokenize(part: &'a str) -> TResult<'a, Self> {
        use crate::parser_utils::*;
        use nom::{branch::alt, bytes::complete::tag, combinator::rest, sequence::preceded};

        let (rem, key_with_value) = preceded(tag(Self::prefix().as_str()), until_newline)(part)?;
        let (value, key) = alt((until_stopbreak_of(":"), rest))(key_with_value)?;
        let value = match value.is_empty() {
            true => None,
            false => Some(value),
        };

        Ok((rem, (key, value).into()))
    }

    //TODO: this should be generated by a concat-related macro, but atm at stable this is not
    //possible, will come back once const generics expands on stable
    fn prefix() -> String {
        format!("{}=", C)
    }
}

impl<'a, const C: char> From<(&'a str, Option<&'a str>)> for Tokenizer<'a, C> {
    fn from((key, value): (&'a str, Option<&'a str>)) -> Self {
        Self { key, value }
    }
}

impl<'a, const C: char> From<(&'a str, &'a str)> for Tokenizer<'a, C> {
    fn from((key, value): (&'a str, &'a str)) -> Self {
        Self {
            key,
            value: Some(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer1() {
        let key_optvalue = concat!("a=recvonly\r\nsomething");

        assert_eq!(
            Tokenizer::<'a'>::tokenize(key_optvalue),
            Ok((
                "something",
                Tokenizer {
                    key: "recvonly",
                    value: None,
                }
            )),
        );
    }

    #[test]
    fn tokenizer2() {
        let key_optvalue = concat!("a=rtpmap:99 h263-1998/90000\r\nsomething");

        assert_eq!(
            Tokenizer::<'a'>::tokenize(key_optvalue),
            Ok((
                "something",
                Tokenizer {
                    key: "rtpmap",
                    value: Some("99 h263-1998/90000"),
                }
            )),
        );
    }
}
