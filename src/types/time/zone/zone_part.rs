use crate::{tokenizers::time::zone::TokenizerPart, types::time::TypedTime, Error};
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ZonePart {
    pub adjustment_time: u64,
    pub offset: TypedTime,
}

impl<'a> TryFrom<TokenizerPart<'a>> for ZonePart {
    type Error = Error;

    fn try_from(tokenizer: TokenizerPart<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            adjustment_time: tokenizer.adjustment.parse()?,
            offset: tokenizer.offset.try_into()?,
        })
    }
}

impl std::fmt::Display for ZonePart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.adjustment_time, self.offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn from_tokenizer1() {
        let tokenizer: TokenizerPart = ("604800", "3600").into();

        assert_eq!(
            ZonePart::try_from(tokenizer),
            Ok(ZonePart {
                adjustment_time: 604800,
                offset: TypedTime::None(Duration::seconds(3600)),
            })
        );
    }

    #[test]
    fn from_tokenizer2() {
        let tokenizer: TokenizerPart = ("604800", "-3h").into();

        assert_eq!(
            ZonePart::try_from(tokenizer),
            Ok(ZonePart {
                adjustment_time: 604800,
                offset: TypedTime::Hours(Duration::hours(-3)),
            })
        );
    }

    #[test]
    fn display1() {
        let zone_part = ZonePart {
            adjustment_time: 604800,
            offset: TypedTime::Hours(Duration::hours(-3)),
        };

        assert_eq!(zone_part.to_string(), "604800 -3h");
    }
}
