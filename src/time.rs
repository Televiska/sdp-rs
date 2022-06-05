use crate::{Active, Error, Repeat, Zone};
use std::convert::{TryFrom, TryInto};

pub use crate::tokenizers::time::Tokenizer;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub struct Time {
    pub active: Active,
    pub repeat: Vec<Repeat>,
    pub zone: Option<Zone>,
}

impl<'a> TryFrom<Tokenizer<'a>> for Time {
    type Error = Error;

    fn try_from(tokenizer: Tokenizer<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            active: tokenizer.active.try_into()?,
            repeat: tokenizer
                .repeat
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            zone: tokenizer.zone.map(TryInto::try_into).transpose()?,
        })
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.active)?;
        for repeat in self.repeat.iter() {
            write!(f, "\r\n{}", repeat)?
        }
        if let Some(zone) = &self.zone {
            write!(f, "\r\n{}", zone)?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn from_tokenizer1() {
        let tokenizer = Tokenizer {
            active: ("3724394400", "3724398000").into(),
            repeat: vec![
                ("604800", "3600", vec!["0"]).into(),
                ("7d", "1h", vec!["0", "25h"]).into(),
            ],
            zone: Some(vec![("3730928400", "-1h"), ("3749680800", "0")].into()),
        };

        assert_eq!(
            Time::try_from(tokenizer),
            Ok(Time {
                active: crate::Active {
                    start: 3724394400,
                    stop: 3724398000,
                },
                repeat: vec![
                    Repeat {
                        interval: crate::TypedTime::None(Duration::seconds(604800)),
                        duration: crate::TypedTime::None(Duration::seconds(3600)),
                        offsets: vec![crate::TypedTime::None(Duration::seconds(0)),],
                    },
                    Repeat {
                        interval: crate::TypedTime::Days(Duration::seconds(604800)),
                        duration: crate::TypedTime::Hours(Duration::seconds(3600)),
                        offsets: vec![
                            crate::TypedTime::None(Duration::seconds(0)),
                            crate::TypedTime::Hours(Duration::seconds(90000)),
                        ],
                    }
                ],
                zone: Some(Zone {
                    parts: vec![
                        crate::ZonePart {
                            adjustment_time: 3730928400,
                            offset: crate::TypedTime::Hours(Duration::hours(-1)),
                        },
                        crate::ZonePart {
                            adjustment_time: 3749680800,
                            offset: crate::TypedTime::None(Duration::hours(0)),
                        },
                    ],
                })
            })
        );
    }

    #[test]
    fn from_tokenizer2() {
        let tokenizer = Tokenizer {
            active: ("3724394400", "3724398000").into(),
            repeat: vec![],
            zone: None,
        };

        assert_eq!(
            Time::try_from(tokenizer),
            Ok(Time {
                active: crate::Active {
                    start: 3724394400,
                    stop: 3724398000,
                },
                repeat: vec![],
                zone: None
            })
        );
    }

    #[test]
    fn display1() {
        let time = Time {
            active: crate::Active {
                start: 3724394400,
                stop: 3724398000,
            },
            repeat: vec![
                Repeat {
                    interval: crate::TypedTime::None(Duration::seconds(604800)),
                    duration: crate::TypedTime::None(Duration::seconds(3600)),
                    offsets: vec![crate::TypedTime::None(Duration::seconds(0))],
                },
                Repeat {
                    interval: crate::TypedTime::Days(Duration::seconds(604800)),
                    duration: crate::TypedTime::Hours(Duration::seconds(3600)),
                    offsets: vec![
                        crate::TypedTime::None(Duration::seconds(0)),
                        crate::TypedTime::Hours(Duration::seconds(90000)),
                    ],
                },
            ],
            zone: Some(Zone {
                parts: vec![
                    crate::ZonePart {
                        adjustment_time: 3730928400,
                        offset: crate::TypedTime::Hours(Duration::hours(-1)),
                    },
                    crate::ZonePart {
                        adjustment_time: 3749680800,
                        offset: crate::TypedTime::None(Duration::hours(0)),
                    },
                ],
            }),
        };

        assert_eq!(
            time.to_string(),
            concat!(
                "t=3724394400 3724398000\r\n",
                "r=604800 3600 0\r\n",
                "r=7d 1h 0 25h\r\n",
                "z=3730928400 -1h 3749680800 0",
            )
        );
    }

    #[test]
    fn display2() {
        let time = Time {
            active: crate::Active {
                start: 3724394400,
                stop: 3724398000,
            },
            repeat: vec![],
            zone: None,
        };

        assert_eq!(time.to_string(), concat!("t=3724394400 3724398000",));
    }
}
