use crate::{tokenizers::time::active::Tokenizer, Error};
use std::convert::TryFrom;

//TODO: convert to Chrono<Utc> ?
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Active {
    pub start: u64,
    pub stop: u64,
}

impl<'a> TryFrom<Tokenizer<'a>> for Active {
    type Error = Error;

    fn try_from(tokenizer: Tokenizer<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            start: tokenizer.start.parse()?,
            stop: tokenizer.stop.parse()?,
        })
    }
}

impl std::fmt::Display for Active {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.start, self.stop)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_tokenizer1() {
        let tokenizer: Tokenizer = ("3724394400", "3724398000").into();

        assert_eq!(
            Active::try_from(tokenizer),
            Ok(Active {
                start: 3724394400,
                stop: 3724398000,
            })
        );
    }

    #[test]
    fn display1() {
        let active = Active {
            start: 3724394400,
            stop: 3724398000,
        };

        assert_eq!(active.to_string(), "3724394400 3724398000");
    }
}
