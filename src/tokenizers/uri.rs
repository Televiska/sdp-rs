use crate::TResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tokenizer<'a> {
    pub value: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a str) -> TResult<'a, Self> {
        use crate::parser_utils::until_newline;
        use nom::{bytes::complete::tag, sequence::preceded};

        let (rem, uri) = preceded(tag("u="), until_newline)(part)?;

        Ok((rem, uri.into()))
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
        let uri = concat!("u=http://www.jdoe.example.com/home.html\r\nsomething",);

        assert_eq!(
            Ok(("something", "http://www.jdoe.example.com/home.html".into())),
            Tokenizer::tokenize(uri)
        );
    }
}
