use crate::Error;
use std::{convert::TryFrom, net::IpAddr};

/// The connection address tokenizer, which is part of the connection (`c=`) line. This is low
/// level stuff and you shouldn't interact directly with it, unless you know what you are doing.
pub use crate::tokenizers::connection::connection_address::Tokenizer;

/// The connection address of the connection line (`c=`).
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Copy)]
pub struct ConnectionAddress {
    pub base: IpAddr,
    pub ttl: Option<u32>,
    pub numaddr: Option<u32>,
}

impl<'a> TryFrom<Tokenizer<'a>> for ConnectionAddress {
    type Error = Error;

    fn try_from(tokenizer: Tokenizer<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            base: tokenizer.base.parse().map_err(|e| {
                Self::Error::parser_with_error("connection address base", tokenizer.base, e)
            })?,
            ttl: tokenizer
                .ttl
                .map(|ttl| {
                    ttl.parse().map_err(|e| {
                        Self::Error::parser_with_error("connection address ttl", ttl, e)
                    })
                })
                .transpose()?,
            numaddr: tokenizer
                .numaddr
                .map(|numaddr| {
                    numaddr.parse().map_err(|e| {
                        Self::Error::parser_with_error("connection number of addresses", numaddr, e)
                    })
                })
                .transpose()?,
        })
    }
}

impl<'a> From<IpAddr> for ConnectionAddress {
    fn from(base: IpAddr) -> Self {
        Self {
            base,
            ttl: None,
            numaddr: None,
        }
    }
}

impl std::fmt::Display for ConnectionAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.ttl, self.numaddr) {
            (Some(ttl), Some(numaddr)) => write!(f, "{}/{}/{}", self.base, ttl, numaddr),
            (Some(ttl), None) => write!(f, "{}/{}", self.base, ttl),
            (None, Some(numaddr)) => write!(f, "{}/{}", self.base, numaddr),
            (None, None) => write!(f, "{}", self.base),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_tokenizer1() {
        let tokenizer: Tokenizer = "233.252.0.1".into();

        assert_eq!(
            ConnectionAddress::try_from(tokenizer),
            Ok(ConnectionAddress {
                base: "233.252.0.1".parse().unwrap(),
                ttl: None,
                numaddr: None
            })
        );
    }

    #[test]
    fn from_tokenizer2() {
        let tokenizer: Tokenizer = ("233.252.0.1", Some("127"), None).into();

        assert_eq!(
            ConnectionAddress::try_from(tokenizer),
            Ok(ConnectionAddress {
                base: "233.252.0.1".parse().unwrap(),
                ttl: Some(127),
                numaddr: None
            })
        );
    }

    #[test]
    fn from_tokenizer3() {
        let tokenizer: Tokenizer = ("233.252.0.1", Some("127"), Some("2")).into();

        assert_eq!(
            ConnectionAddress::try_from(tokenizer),
            Ok(ConnectionAddress {
                base: "233.252.0.1".parse().unwrap(),
                ttl: Some(127),
                numaddr: Some(2)
            })
        );
    }

    #[test]
    fn display1() {
        let connection = ConnectionAddress {
            base: "233.252.0.1".parse().unwrap(),
            ttl: Some(127),
            numaddr: Some(2),
        };

        assert_eq!(connection.to_string(), "233.252.0.1/127/2");
    }
}
