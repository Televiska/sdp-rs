//! Types related to the zone line (`z=`).

mod zone_part;

/// The zone line (`r=`) tokenizer. This is low level stuff and you shouldn't interact directly
/// with it, unless you know what you are doing.
pub use crate::tokenizers::time::zone::Tokenizer;
pub use zone_part::ZonePart;

use crate::Error;
use std::convert::{TryFrom, TryInto};

/// A zone line (`z=`) of SDP. It holds all the repeat zone adjustments in a `Vector`.
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub struct Zone {
    pub parts: Vec<ZonePart>,
}

impl<'a> TryFrom<Tokenizer<'a>> for Zone {
    type Error = Error;

    fn try_from(tokenizer: Tokenizer<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            parts: tokenizer
                .parts
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, Error>>()?,
        })
    }
}

impl std::fmt::Display for Zone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "z={}",
            self.parts
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lines::common::TypedTime;
    use chrono::Duration;

    #[test]
    fn from_tokenizer1() {
        let tokenizer: Tokenizer = vec![("3730928400", "-1h"), ("3749680800", "0")].into();

        assert_eq!(
            Zone::try_from(tokenizer),
            Ok(Zone {
                parts: vec![
                    ZonePart {
                        adjustment_time: 3730928400,
                        offset: TypedTime::Hours(Duration::hours(-1)),
                    },
                    ZonePart {
                        adjustment_time: 3749680800,
                        offset: TypedTime::None(Duration::hours(0)),
                    }
                ],
            })
        );
    }

    #[test]
    fn display1() {
        let zone = Zone {
            parts: vec![
                ZonePart {
                    adjustment_time: 3730928400,
                    offset: TypedTime::Hours(Duration::hours(-1)),
                },
                ZonePart {
                    adjustment_time: 3749680800,
                    offset: TypedTime::None(Duration::hours(0)),
                },
            ],
        };

        assert_eq!(zone.to_string(), "z=3730928400 -1h 3749680800 0");
    }
}
