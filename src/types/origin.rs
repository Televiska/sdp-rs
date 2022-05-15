use crate::{
    types::common::{Addrtype, Nettype},
    {tokenizers::origin::Tokenizer, Error},
};
use std::{convert::TryFrom, net::IpAddr};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Origin {
    pub username: String,
    pub sess_id: String,
    pub sess_version: String,
    pub nettype: Nettype,
    pub addrtype: Addrtype,
    pub unicast_address: IpAddr,
}

impl<'a> TryFrom<Tokenizer<'a>> for Origin {
    type Error = Error;

    fn try_from(tokenizer: Tokenizer<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            username: tokenizer.username.into(),
            sess_id: tokenizer.sess_id.into(),
            sess_version: tokenizer.sess_version.into(),
            nettype: tokenizer.nettype.into(),
            addrtype: tokenizer.addrtype.into(),
            unicast_address: tokenizer.unicast_address.parse()?,
        })
    }
}

impl std::fmt::Display for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "o={} {} {} {} {} {}",
            self.username,
            self.sess_id,
            self.sess_version,
            self.nettype,
            self.addrtype,
            self.unicast_address
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn from_tokenizer1() {
        let tokenizer = Tokenizer {
            username: "jdoe",
            sess_id: "3724394400",
            sess_version: "3724394405",
            nettype: "IN",
            addrtype: "IP4",
            unicast_address: "198.51.100.1",
        };

        assert_eq!(
            Origin::try_from(tokenizer),
            Ok(Origin {
                username: "jdoe".into(),
                sess_id: "3724394400".into(),
                sess_version: "3724394405".into(),
                nettype: Nettype::In,
                addrtype: Addrtype::Ip4,
                unicast_address: IpAddr::V4(Ipv4Addr::new(198, 51, 100, 1)),
            })
        );
    }

    #[test]
    fn from_tokenizer2() {
        let tokenizer = Tokenizer {
            username: "jdoe",
            sess_id: "3724394400",
            sess_version: "3724394405",
            nettype: "IN",
            addrtype: "IP4",
            unicast_address: "noip",
        };

        assert!(Origin::try_from(tokenizer).is_err());
    }

    #[test]
    fn display1() {
        let origin = Origin {
            username: "jdoe".into(),
            sess_id: "3724394400".into(),
            sess_version: "3724394405".into(),
            nettype: Nettype::In,
            addrtype: Addrtype::Ip4,
            unicast_address: IpAddr::V4(Ipv4Addr::new(198, 51, 100, 1)),
        };

        assert_eq!(
            origin.to_string(),
            "o=jdoe 3724394400 3724394405 IN IP4 198.51.100.1"
        );
    }
}
