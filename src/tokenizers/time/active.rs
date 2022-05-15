use crate::TResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tokenizer<'a> {
    pub start: &'a str,
    pub stop: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a str) -> TResult<'a, Self> {
        use crate::parser_utils::*;
        use nom::{bytes::complete::tag, sequence::preceded};

        let (rem, start) = preceded(tag("t="), until_space)(part)?;
        let (rem, stop) = until_newline(rem)?;

        Ok((rem, (start, stop).into()))
    }
}

impl<'a> From<(&'a str, &'a str)> for Tokenizer<'a> {
    fn from((start, stop): (&'a str, &'a str)) -> Self {
        Self { start, stop }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer() {
        let time = concat!("t=3724394400 3724398000\r\nsomething");

        assert_eq!(
            Tokenizer::tokenize(time),
            Ok(("something", ("3724394400", "3724398000").into())),
        );
    }
}
