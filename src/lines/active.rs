//! Types related to the time active line (`a=`).

use crate::Error;
use std::convert::TryFrom;

/// The time active line (`a=`) tokenizer. This is low level stuff and you shouldn't interact
/// directly with it, unless you know what you are doing.
pub use crate::tokenizers::time::active::Tokenizer;

/// The time active (`a=`) of SDP. `start` and `stop` are saved as the appear in SDP, you will need
/// `chrono` (or `std`) to convert to an actual time.
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Copy)]
pub struct Active {
    pub start: u64,
    pub stop: u64,
}

impl<'a> TryFrom<Tokenizer<'a>> for Active {
    type Error = Error;

    fn try_from(tokenizer: Tokenizer<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            start: tokenizer
                .start
                .parse()
                .map_err(|e| Self::Error::parser_with_error("time start", tokenizer.start, e))?,
            stop: tokenizer
                .stop
                .parse()
                .map_err(|e| Self::Error::parser_with_error("time stop", tokenizer.stop, e))?,
        })
    }
}

impl std::fmt::Display for Active {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "t={} {}", self.start, self.stop)
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

        assert_eq!(active.to_string(), "t=3724394400 3724398000");
    }
}
