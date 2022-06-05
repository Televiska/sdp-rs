use crate::Error;
use std::convert::TryFrom;

pub use crate::tokenizers::value::Tokenizer;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Copy)]
pub enum Version {
    V0,
}

impl<'a> TryFrom<Tokenizer<'a, 'v'>> for Version {
    type Error = Error;

    fn try_from(tokenizer: Tokenizer<'a, 'v'>) -> Result<Self, Self::Error> {
        if tokenizer.value.eq("0") {
            Ok(Self::V0)
        } else {
            Err(Error::parser("version", tokenizer.value))
        }
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::V0 => write!(f, "v=0"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_tokenizer1() {
        let tokenizer: Tokenizer<'v'> = "0".into();

        assert_eq!(Version::try_from(tokenizer), Ok(Version::V0));
    }

    #[test]
    fn from_tokenizer2() {
        let tokenizer: Tokenizer<'v'> = "1".into();

        assert!(Version::try_from(tokenizer).is_err());
    }

    #[test]
    fn display1() {
        let version = Version::V0;

        assert_eq!(version.to_string(), "v=0");
    }
}
