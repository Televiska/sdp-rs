use super::zone_part;
use crate::TResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tokenizer<'a> {
    pub parts: Vec<zone_part::Tokenizer<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a str) -> TResult<'a, Self> {
        use crate::parser_utils::*;
        use nom::{bytes::complete::tag, multi::many1, sequence::preceded};

        let (rem, line) = preceded(tag("z="), until_newline)(part)?;
        let (_, parts) = many1(zone_part::Tokenizer::tokenize)(line)?;

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

impl<'a, T: Into<zone_part::Tokenizer<'a>>> From<Vec<T>> for Tokenizer<'a> {
    fn from(parts: Vec<T>) -> Self {
        Self {
            parts: parts.into_iter().map(Into::into).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer1() {
        let zone = concat!("z=3730928400 -1h\r\nsomething");

        assert_eq!(
            Tokenizer::tokenize(zone),
            Ok(("something", ("3730928400", "-1h").into())),
        );
    }

    #[test]
    fn tokenizer2() {
        let zone = concat!("z=3730928400 -1h 3749680800 0\r\nsomething");

        assert_eq!(
            Tokenizer::tokenize(zone),
            Ok((
                "something",
                vec![("3730928400", "-1h"), ("3749680800", "0")].into()
            )),
        );
    }

    #[test]
    fn tokenizer3() {
        let zone = concat!("z=3730928400 -1h 3749680800 0h\r\nsomething");

        assert_eq!(
            Tokenizer::tokenize(zone),
            Ok((
                "something",
                vec![("3730928400", "-1h"), ("3749680800", "0h")].into()
            )),
        );
    }
}
