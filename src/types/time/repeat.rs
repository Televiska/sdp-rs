use crate::{tokenizers::time::repeat::Tokenizer, Error};
use std::convert::{TryFrom, TryInto};

use super::TypedTime;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Repeat {
    pub interval: TypedTime,
    pub duration: TypedTime,
    pub offsets: Vec<TypedTime>,
}

impl<'a> TryFrom<Tokenizer<'a>> for Repeat {
    type Error = Error;

    fn try_from(tokenizer: Tokenizer<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            interval: tokenizer.interval.try_into()?,
            duration: tokenizer.duration.try_into()?,
            offsets: tokenizer
                .offsets
                .into_iter()
                .map(TryInto::<TypedTime>::try_into)
                .collect::<Result<Vec<_>, Error>>()?,
        })
    }
}

impl std::fmt::Display for Repeat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.interval,
            self.duration,
            self.offsets
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn from_tokenizer1() {
        let tokenizer: Tokenizer = ("604800", "3600", vec!["0", "90000"]).into();

        assert_eq!(
            Repeat::try_from(tokenizer),
            Ok(Repeat {
                interval: TypedTime::None(Duration::seconds(604800)),
                duration: TypedTime::None(Duration::seconds(3600)),
                offsets: vec![
                    TypedTime::None(Duration::seconds(0)),
                    TypedTime::None(Duration::seconds(90000))
                ]
            })
        );
    }

    #[test]
    fn from_tokenizer2() {
        let tokenizer: Tokenizer = ("7d", "1h", vec!["0", "25h"]).into();

        assert_eq!(
            Repeat::try_from(tokenizer),
            Ok(Repeat {
                interval: TypedTime::Days(Duration::seconds(604800)),
                duration: TypedTime::Hours(Duration::seconds(3600)),
                offsets: vec![
                    TypedTime::None(Duration::seconds(0)),
                    TypedTime::Hours(Duration::seconds(90000))
                ]
            })
        );
    }

    #[test]
    fn display1() {
        let repeat = Repeat {
            interval: TypedTime::Days(Duration::seconds(604800)),
            duration: TypedTime::Hours(Duration::seconds(3600)),
            offsets: vec![
                TypedTime::None(Duration::seconds(0)),
                TypedTime::Hours(Duration::seconds(90000)),
            ],
        };

        assert_eq!(repeat.to_string(), "7d 1h 0 25h");
    }

    #[test]
    fn display2() {
        let repeat = Repeat {
            interval: TypedTime::None(Duration::seconds(604800)),
            duration: TypedTime::None(Duration::seconds(3600)),
            offsets: vec![
                TypedTime::None(Duration::seconds(0)),
                TypedTime::None(Duration::seconds(90000)),
            ],
        };

        assert_eq!(repeat.to_string(), "604800 3600 0 90000");
    }
}
