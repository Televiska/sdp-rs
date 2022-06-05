mod connection_address;

pub use crate::tokenizers::connection::Tokenizer;
pub use connection_address::ConnectionAddress;

use crate::{
    types::common::{Addrtype, Nettype},
    Error,
};
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub struct Connection {
    pub nettype: Nettype,
    pub addrtype: Addrtype,
    pub connection_address: ConnectionAddress,
}

impl<'a> TryFrom<Tokenizer<'a>> for Connection {
    type Error = Error;

    fn try_from(tokenizer: Tokenizer<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            nettype: tokenizer.nettype.into(),
            addrtype: tokenizer.addrtype.into(),
            connection_address: tokenizer.connection_address.try_into()?,
        })
    }
}

impl std::fmt::Display for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "c={} {} {}",
            self.nettype, self.addrtype, self.connection_address
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_tokenizer1() {
        let tokenizer: Tokenizer = ("IN", "IP4", ("233.252.0.1")).into();

        assert_eq!(
            Connection::try_from(tokenizer),
            Ok(Connection {
                nettype: Nettype::In,
                addrtype: Addrtype::Ip4,
                connection_address: ConnectionAddress {
                    base: "233.252.0.1".parse().unwrap(),
                    ttl: None,
                    numaddr: None
                }
            })
        );
    }

    #[test]
    fn from_tokenizer2() {
        let tokenizer: Tokenizer = ("In", "Ip4", ("233.252.0.1", "127", "2")).into();

        assert_eq!(
            Connection::try_from(tokenizer),
            Ok(Connection {
                nettype: Nettype::Other("In".into()),
                addrtype: Addrtype::Other("Ip4".into()),
                connection_address: ConnectionAddress {
                    base: "233.252.0.1".parse().unwrap(),
                    ttl: Some(127),
                    numaddr: Some(2)
                }
            })
        );
    }

    #[test]
    fn display1() {
        let connection = Connection {
            nettype: Nettype::In,
            addrtype: Addrtype::Other("Ip4".into()),
            connection_address: ConnectionAddress {
                base: "233.252.0.1".parse().unwrap(),
                ttl: Some(127),
                numaddr: Some(2),
            },
        };

        assert_eq!(connection.to_string(), "c=IN Ip4 233.252.0.1/127/2");
    }
}
