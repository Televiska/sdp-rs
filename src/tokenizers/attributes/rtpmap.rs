use crate::TResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tokenizer<'a> {
    pub payload_type: &'a str,
    pub encoding_name: &'a str,
    pub clock_rate: &'a str,
    pub encoding_params: Option<&'a str>,
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a str) -> TResult<'a, Self> {
        use crate::parser_utils::*;
        use nom::{branch::alt, combinator::rest};

        let (rem, payload_type) = until_space(part)?;
        let (rem, encoding_name) = until_stopbreak_of("/")(rem)?;
        let (encoding_params, clock_rate) = alt((until_stopbreak_of("/"), rest))(rem)?;

        Ok((
            "",
            Tokenizer {
                payload_type,
                encoding_name,
                clock_rate,
                encoding_params: (!encoding_params.is_empty()).then(|| encoding_params),
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer() {
        let rtpmap = "98 L16/16000/2";

        assert_eq!(
            Tokenizer::tokenize(rtpmap),
            Ok((
                "",
                Tokenizer {
                    payload_type: "98",
                    encoding_name: "L16",
                    clock_rate: "16000",
                    encoding_params: Some("2"),
                }
            )),
        );
    }

    #[test]
    fn tokenizer2() {
        let rtpmap = "96 L8/8000";

        assert_eq!(
            Tokenizer::tokenize(rtpmap),
            Ok((
                "",
                Tokenizer {
                    payload_type: "96",
                    encoding_name: "L8",
                    clock_rate: "8000",
                    encoding_params: None,
                }
            )),
        );
    }
}
