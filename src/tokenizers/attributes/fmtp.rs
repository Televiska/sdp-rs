use crate::TResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tokenizer<'a> {
    pub fmt: &'a str,
    pub params: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a str) -> TResult<'a, Self> {
        use crate::parser_utils::*;

        let (params, fmt) = until_space(part)?;

        Ok(("", Tokenizer { fmt, params }))
    }
}

impl<'a> From<(&'a str, &'a str)> for Tokenizer<'a> {
    fn from((fmt, params): (&'a str, &'a str)) -> Self {
        Self { fmt, params }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer() {
        let rtpmap = "96 profile-level-id=42e016;max-mbps=108000;max-fs=3600";

        assert_eq!(
            Tokenizer::tokenize(rtpmap),
            Ok((
                "",
                Tokenizer {
                    fmt: "96",
                    params: "profile-level-id=42e016;max-mbps=108000;max-fs=3600",
                }
            )),
        );
    }
}
