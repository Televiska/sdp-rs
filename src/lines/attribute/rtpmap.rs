/// The rtpmap tokenizer, which is part of the attribute (`a=`) line. This is low
/// level stuff and you shouldn't interact directly with it, unless you know what you are doing.
pub use crate::tokenizers::attributes::rtpmap::Tokenizer;
use std::convert::TryFrom;

/// The `rtpmap` attribute as it appears in the attribute line(s) (`a=`).
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub struct Rtpmap {
    pub payload_type: u32,
    pub encoding_name: String,
    pub clock_rate: i32,
    pub encoding_params: Option<i32>,
}

impl<'a> TryFrom<Tokenizer<'a>> for Rtpmap {
    type Error = crate::Error;

    fn try_from(tokenizer: Tokenizer<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            payload_type: tokenizer.payload_type.parse()?,
            encoding_name: tokenizer.encoding_name.into(),
            clock_rate: tokenizer.clock_rate.parse()?,
            encoding_params: tokenizer.encoding_params.map(|s| s.parse()).transpose()?,
        })
    }
}

impl<'a> TryFrom<&'a str> for Rtpmap {
    type Error = crate::Error;

    fn try_from(part: &'a str) -> Result<Self, Self::Error> {
        Self::try_from(Tokenizer::tokenize(part)?.1)
    }
}

impl std::fmt::Display for Rtpmap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.encoding_params {
            Some(encoding_params) => write!(
                f,
                "{} {}/{} {}",
                self.payload_type, self.encoding_name, self.clock_rate, encoding_params
            ),
            None => write!(
                f,
                "{} {}/{}",
                self.payload_type, self.encoding_name, self.clock_rate
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str1() {
        let tokenizer = Tokenizer {
            payload_type: "96",
            encoding_name: "L8",
            clock_rate: "8000",
            encoding_params: None,
        };

        assert_eq!(
            Rtpmap::try_from(tokenizer),
            Ok(Rtpmap {
                payload_type: 96,
                encoding_name: "L8".into(),
                clock_rate: 8000,
                encoding_params: None,
            })
        );
    }

    #[test]
    fn from_str2() {
        let tokenizer = Tokenizer {
            payload_type: "98",
            encoding_name: "L16",
            clock_rate: "16000",
            encoding_params: Some("2"),
        };

        assert_eq!(
            Rtpmap::try_from(tokenizer),
            Ok(Rtpmap {
                payload_type: 98,
                encoding_name: "L16".into(),
                clock_rate: 16000,
                encoding_params: Some(2),
            })
        );
    }

    #[test]
    fn display1() {
        assert_eq!(
            Rtpmap {
                payload_type: 98,
                encoding_name: "L16".into(),
                clock_rate: 16000,
                encoding_params: Some(2),
            }
            .to_string(),
            "98 L16/16000 2"
        );
    }
}
