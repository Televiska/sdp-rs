use crate::{types::common::Bwtype, Error};
use std::convert::TryFrom;

pub use crate::tokenizers::key_value::Tokenizer;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub struct Bandwidth {
    pub bwtype: Bwtype,
    pub bandwidth: u32,
}

impl<'a> TryFrom<Tokenizer<'a, 'b'>> for Bandwidth {
    type Error = Error;

    fn try_from(tokenizer: Tokenizer<'a, 'b'>) -> Result<Self, Self::Error> {
        Ok(Self {
            bwtype: tokenizer.key.into(),
            bandwidth: tokenizer.value.parse().map_err(|e| {
                Self::Error::parser_with_error("bandwidth value", tokenizer.value, e)
            })?,
        })
    }
}

impl std::fmt::Display for Bandwidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "b={}:{}", self.bwtype, self.bandwidth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_tokenizer1() {
        let tokenizer: Tokenizer<'b'> = ("AS", "80").into();

        assert_eq!(
            Bandwidth::try_from(tokenizer),
            Ok(Bandwidth {
                bwtype: Bwtype::As,
                bandwidth: 80,
            })
        );
    }

    #[test]
    fn display1() {
        let bandwidth = Bandwidth {
            bwtype: Bwtype::As,
            bandwidth: 80,
        };

        assert_eq!(bandwidth.to_string(), "b=AS:80");
    }
}
