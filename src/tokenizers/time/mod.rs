pub mod active;
pub mod repeat;
pub mod zone;
pub mod zone_part;

use crate::TResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tokenizer<'a> {
    pub active: active::Tokenizer<'a>,
    pub repeat: Vec<repeat::Tokenizer<'a>>,
    pub zone: Option<zone::Tokenizer<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a str) -> TResult<'a, Self> {
        use nom::multi::many1;

        let (rem, active) = active::Tokenizer::tokenize(part)?;
        let (rem, repeat) = match rem.starts_with("r=") {
            true => many1(repeat::Tokenizer::tokenize)(rem)?,
            false => (rem, vec![]),
        };
        let (rem, zone) = match rem.starts_with("z=") {
            true => {
                let (rem, info) = zone::Tokenizer::tokenize(rem)?;
                (rem, Some(info))
            }
            false => (rem, None),
        };

        Ok((
            rem,
            Self {
                active,
                repeat,
                zone,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer1() {
        let time = concat!(
            "t=3724394400 3724398000\r\n",
            "r=604800 3600 0\r\n",
            "r=7d 1h 0 25h\r\n",
            "z=3730928400 -1h 3749680800 0\r\n",
            "something"
        );

        assert_eq!(
            Tokenizer::tokenize(time),
            Ok((
                "something",
                Tokenizer {
                    active: ("3724394400", "3724398000").into(),
                    repeat: vec![
                        ("604800", "3600", vec!["0"]).into(),
                        ("7d", "1h", vec!["0", "25h"]).into()
                    ],
                    zone: Some(vec![("3730928400", "-1h"), ("3749680800", "0")].into())
                }
            ))
        );
    }

    #[test]
    fn tokenizer2() {
        let time = concat!("t=3724394400 3724398000\r\nsomething");

        assert_eq!(
            Tokenizer::tokenize(time),
            Ok((
                "something",
                Tokenizer {
                    active: ("3724394400", "3724398000").into(),
                    repeat: vec![],
                    zone: None
                }
            ))
        );
    }
}
