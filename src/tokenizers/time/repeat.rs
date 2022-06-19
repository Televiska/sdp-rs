use crate::TResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tokenizer<'a> {
    pub interval: &'a str,
    pub duration: &'a str,
    pub offsets: Vec<&'a str>,
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a str) -> TResult<'a, Self> {
        use crate::parser_utils::*;
        use nom::{bytes::complete::tag, multi::many0, sequence::preceded};

        let (rem, line) = preceded(tag("r="), until_newline)(part)?;
        let (line_rem, interval) = until_space(line)?;
        let (offsets, duration) = until_space(line_rem)?;
        let (offset, mut offsets) = many0(until_space)(offsets)?;

        offsets.push(offset);

        Ok((rem, (interval, duration, offsets).into()))
    }
}

impl<'a> From<(&'a str, &'a str, Vec<&'a str>)> for Tokenizer<'a> {
    fn from((interval, duration, offsets): (&'a str, &'a str, Vec<&'a str>)) -> Self {
        Self {
            interval,
            duration,
            offsets,
        }
    }
}

impl<'a> From<(&'a str, &'a str)> for Tokenizer<'a> {
    fn from((interval, duration): (&'a str, &'a str)) -> Self {
        Self {
            interval,
            duration,
            offsets: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer1() {
        let repeat = concat!("r=604800 3600 0\r\nsomething");

        assert_eq!(
            Ok(("something", ("604800", "3600", vec!["0"]).into())),
            Tokenizer::tokenize(repeat)
        );
    }

    #[test]
    fn tokenizer2() {
        let repeat = concat!("r=604800 3600 0 90000\r\nsomething");

        assert_eq!(
            Tokenizer::tokenize(repeat),
            Ok(("something", ("604800", "3600", vec!["0", "90000"]).into())),
        );
    }

    #[test]
    fn tokenizer3() {
        let repeat = concat!("r=7d 1h 0 25h\r\nsomething");

        assert_eq!(
            Tokenizer::tokenize(repeat),
            Ok(("something", ("7d", "1h", vec!["0", "25h"]).into())),
        );
    }
}
