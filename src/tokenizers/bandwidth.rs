use crate::TResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tokenizer<'a> {
    pub bwtype: &'a str,
    pub bandwidth: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a str) -> TResult<'a, Self> {
        use crate::parser_utils::*;
        use nom::{bytes::complete::tag, sequence::preceded};

        let (rem, bwtype) = preceded(tag("b="), until_stopbreak_of(":"))(part)?;
        let (rem, bandwidth) = until_newline(rem)?;

        Ok((rem, (bwtype, bandwidth).into()))
    }
}

impl<'a> From<(&'a str, &'a str)> for Tokenizer<'a> {
    fn from((bwtype, bandwidth): (&'a str, &'a str)) -> Self {
        Self { bwtype, bandwidth }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer() {
        let bandwidth = concat!("b=CT:128\r\nsomething");

        assert_eq!(
            Ok((
                "something",
                Tokenizer {
                    bwtype: "CT",
                    bandwidth: "128",
                }
            )),
            Tokenizer::tokenize(bandwidth)
        );
    }
}
