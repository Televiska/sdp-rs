use crate::TResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tokenizer<'a> {
    pub value: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a str) -> TResult<'a, Self> {
        use crate::parser_utils::until_newline;
        use nom::{bytes::complete::tag, sequence::preceded};

        let (rem, session_name) = preceded(tag("s="), until_newline)(part)?;

        Ok((rem, session_name.into()))
    }
}

impl<'a> From<&'a str> for Tokenizer<'a> {
    fn from(value: &'a str) -> Self {
        Self { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer() {
        let session_name = concat!("s=Call to John Smith\r\nsomething",);

        assert_eq!(
            Ok(("something", "Call to John Smith".into())),
            Tokenizer::tokenize(session_name)
        );
    }
}
