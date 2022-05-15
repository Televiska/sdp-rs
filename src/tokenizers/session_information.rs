use crate::TResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tokenizer<'a> {
    pub value: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a str) -> TResult<'a, Self> {
        use crate::parser_utils::until_newline;
        use nom::{bytes::complete::tag, sequence::preceded};

        let (rem, session_information) = preceded(tag("i="), until_newline)(part)?;

        Ok((rem, session_information.into()))
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
        let session_information = concat!("i=SDP Offer #1\r\nsomething",);

        assert_eq!(
            Ok(("something", "SDP Offer #1".into())),
            Tokenizer::tokenize(session_information)
        );
    }
}
