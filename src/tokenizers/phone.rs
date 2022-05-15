use crate::TResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tokenizer<'a> {
    pub value: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a str) -> TResult<'a, Self> {
        use crate::parser_utils::until_newline;
        use nom::{bytes::complete::tag, sequence::preceded};

        let (rem, phone) = preceded(tag("p="), until_newline)(part)?;

        Ok((rem, phone.into()))
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
        let phone = concat!("p=+1 617 555-6011\r\nsomething",);

        assert_eq!(
            Ok(("something", "+1 617 555-6011".into())),
            Tokenizer::tokenize(phone)
        );
    }
}
