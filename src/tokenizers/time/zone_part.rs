use crate::TResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tokenizer<'a> {
    pub adjustment: &'a str,
    pub offset: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a str) -> TResult<'a, Self> {
        use crate::parser_utils::*;
        use nom::branch::alt;
        use nom::combinator::rest;

        let (rem, adjustment) = until_space(part)?;
        let (rem, offset) = alt((until_space, rest))(rem)?;

        Ok((rem, Self { adjustment, offset }))
    }
}

impl<'a> From<(&'a str, &'a str)> for Tokenizer<'a> {
    fn from((adjustment, offset): (&'a str, &'a str)) -> Self {
        Self { adjustment, offset }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer_part1() {
        let zone_part = concat!("3730928400 -1h");

        assert_eq!(
            Tokenizer::tokenize(zone_part),
            Ok(("", ("3730928400", "-1h").into())),
        );
    }

    #[test]
    fn tokenizer_part2() {
        let zone_part = concat!("3730928400 0h");

        assert_eq!(
            Tokenizer::tokenize(zone_part),
            Ok(("", ("3730928400", "0h").into())),
        );
    }
}
