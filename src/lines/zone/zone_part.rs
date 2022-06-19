use crate::{lines::common::TypedTime, Error};
use std::convert::{TryFrom, TryInto};

pub use crate::tokenizers::time::zone_part::Tokenizer;

/// A zone part is part of the zone line ([super::Zone]). It holds the adjustment time and the
/// offset of that adjustment time.
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Copy)]
pub struct ZonePart {
    pub adjustment_time: u64,
    pub offset: TypedTime,
}

impl<'a> TryFrom<Tokenizer<'a>> for ZonePart {
    type Error = Error;

    fn try_from(tokenizer: Tokenizer<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            adjustment_time: tokenizer.adjustment.parse().map_err(|e| {
                Self::Error::parser_with_error("zone adjustment time", tokenizer.adjustment, e)
            })?,
            offset: tokenizer
                .offset
                .try_into()
                .map_err(|e| Self::Error::parser_with_error("zone offset", tokenizer.offset, e))?,
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
        let tokenizer: Tokenizer = ("604800", "3600").into();

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
        let tokenizer: Tokenizer = ("604800", "-3h").into();

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
