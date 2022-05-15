use crate::TResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tokenizer<'a> {
    pub parts: Vec<TokenizerPart<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a str) -> TResult<'a, Self> {
        use nom::{bytes::complete::tag, multi::many1};

        let (rem, _) = tag("z=")(part)?;
        let (rem, parts) = many1(TokenizerPart::tokenize)(rem)?;

        Ok((rem, Self { parts }))
    }
}

impl<'a> From<(&'a str, &'a str)> for Tokenizer<'a> {
    fn from((adjustment, offset): (&'a str, &'a str)) -> Self {
        Self {
            parts: vec![(adjustment, offset).into()],
        }
    }
}

impl<'a, T: Into<TokenizerPart<'a>>> From<Vec<T>> for Tokenizer<'a> {
    fn from(parts: Vec<T>) -> Self {
        Self {
            parts: parts.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TokenizerPart<'a> {
    pub adjustment: &'a str,
    pub offset: &'a str,
}

impl<'a> TokenizerPart<'a> {
    pub fn tokenize(part: &'a str) -> TResult<'a, Self> {
        use crate::parser_utils::*;
        use nom::branch::alt;

        let (rem, adjustment) = until_space(part)?;
        let (rem, offset) = alt((until_space, until_newline))(rem)?;

        Ok((rem, Self { adjustment, offset }))
    }
}

impl<'a> From<(&'a str, &'a str)> for TokenizerPart<'a> {
    fn from((adjustment, offset): (&'a str, &'a str)) -> Self {
        Self { adjustment, offset }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer_part1() {
        let zone = concat!("3730928400 -1h\r\nsomething");

        assert_eq!(
            Ok(("something", ("3730928400", "-1h").into())),
            TokenizerPart::tokenize(zone)
        );
    }

    #[test]
    fn tokenizer1() {
        let zone = concat!("z=3730928400 -1h\r\nsomething");

        assert_eq!(
            Ok(("something", ("3730928400", "-1h").into())),
            Tokenizer::tokenize(zone)
        );
    }

    #[test]
    fn tokenizer2() {
        let zone = concat!("z=3730928400 -1h 3749680800 0\r\nsomething");

        assert_eq!(
            Ok((
                "something",
                vec![("3730928400", "-1h"), ("3749680800", "0")].into()
            )),
            Tokenizer::tokenize(zone)
        );
    }
}
