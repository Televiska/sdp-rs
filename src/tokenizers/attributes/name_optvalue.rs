use crate::{TResult, TokenizerError};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tokenizer<'a> {
    pub name: &'a str,
    pub value: Option<&'a str>,
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a str) -> TResult<'a, Self> {
        use crate::parser_utils::*;
        use nom::{branch::alt, combinator::rest};

        if part.is_empty() {
            return Err(nom::Err::Error(TokenizerError::from(
                "nothing more to tokenize",
            )));
        }

        let (rem, name_with_value) = alt((until_stopbreak_of(";"), rest))(part)?;
        let (name, value) = match name_with_value.contains('=') {
            true => {
                let (value, name) = until_stopbreak_of("=")(name_with_value)?;
                (name, Some(value))
            }
            false => (name_with_value, None),
        };

        println!("{} -- {:?}", name, value);

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
        let input = concat!("something");

        assert_eq!(
            Tokenizer::tokenize(input),
            Ok((
                "",
                Tokenizer {
                    name: "something",
                    value: None,
                }
            )),
        );
    }

    #[test]
    fn tokenizer2() {
        let input = concat!("something;");

        assert_eq!(
            Tokenizer::tokenize(input),
            Ok((
                "",
                Tokenizer {
                    name: "something",
                    value: None,
                }
            )),
        );
    }

    #[test]
    fn tokenizer3() {
        let input = concat!("something=else");

        assert_eq!(
            Tokenizer::tokenize(input),
            Ok((
                "",
                Tokenizer {
                    name: "something",
                    value: Some("else"),
                }
            )),
        );
    }

    #[test]
    fn tokenizer4() {
        let input = concat!("something=else;");

        assert_eq!(
            Tokenizer::tokenize(input),
            Ok((
                "",
                Tokenizer {
                    name: "something",
                    value: Some("else"),
                }
            )),
        );
    }
}
