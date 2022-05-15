use crate::TResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tokenizer<'a> {
    pub method: &'a str,
    pub key: Option<&'a str>,
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a str) -> TResult<'a, Self> {
        use crate::parser_utils::*;
        use nom::{branch::alt, bytes::complete::tag, combinator::rest, sequence::preceded};

        let (rem, method_with_key) = preceded(tag("k="), until_newline)(part)?;
        let (key, method) = alt((until_stopbreak_of(":"), rest))(method_with_key)?;
        let key = match key.is_empty() {
            true => None,
            false => Some(key),
        };

        Ok((rem, (method, key).into()))
    }
}

impl<'a> From<(&'a str, Option<&'a str>)> for Tokenizer<'a> {
    fn from((method, key): (&'a str, Option<&'a str>)) -> Self {
        Self { method, key }
    }
}

impl<'a> From<(&'a str, &'a str)> for Tokenizer<'a> {
    fn from((method, key): (&'a str, &'a str)) -> Self {
        Self {
            method,
            key: Some(key),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer1() {
        let encryption_key = concat!("k=prompt\r\nsomething");

        assert_eq!(
            Ok((
                "something",
                Tokenizer {
                    method: "prompt",
                    key: None,
                }
            )),
            Tokenizer::tokenize(encryption_key)
        );
    }

    #[test]
    fn tokenizer2() {
        let encryption_key =
            concat!("k=base64:acx4fimF1pQdu6y2QTzttXjr5Z3eOVmmVu4YRZQoKqc=\r\nsomething");

        assert_eq!(
            Ok((
                "something",
                Tokenizer {
                    method: "base64",
                    key: Some("acx4fimF1pQdu6y2QTzttXjr5Z3eOVmmVu4YRZQoKqc="),
                }
            )),
            Tokenizer::tokenize(encryption_key)
        );
    }
}
