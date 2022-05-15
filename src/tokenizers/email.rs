use crate::TResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tokenizer<'a> {
    pub value: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a str) -> TResult<'a, Self> {
        use crate::parser_utils::until_newline;
        use nom::{bytes::complete::tag, sequence::preceded};

        let (rem, email) = preceded(tag("e="), until_newline)(part)?;

        Ok((rem, email.into()))
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
        let email = concat!("e=j.doe@example.com (Jane Doe)\r\nsomething",);

        assert_eq!(
            Ok(("something", "j.doe@example.com (Jane Doe)".into())),
            Tokenizer::tokenize(email)
        );
    }
}
