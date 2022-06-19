/// The Addrtype as it appears in the connection or origin lines. It's not a `Copy` type since it
/// supports abstract types, not even defined in any RFC.
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub enum Addrtype {
    Ip4,
    Ip6,
    Other(String),
}

impl<'a> From<&'a str> for Addrtype {
    fn from(s: &str) -> Self {
        match s {
            s if s.eq("IP4") => Self::Ip4,
            s if s.eq("IP6") => Self::Ip6,
            _ => Self::Other(s.into()),
        }
    }
}

impl std::fmt::Display for Addrtype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ip4 => write!(f, "IP4",),
            Self::Ip6 => write!(f, "IP6",),
            Self::Other(other) => write!(f, "{}", other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str1() {
        let part = "IP4";

        assert_eq!(Addrtype::from(part), Addrtype::Ip4);
    }

    #[test]
    fn from_str2() {
        let part = "IP6";

        assert_eq!(Addrtype::from(part), Addrtype::Ip6);
    }

    #[test]
    fn from_str3() {
        let part = "ip4";

        assert_eq!(Addrtype::from(part), Addrtype::Other("ip4".into()));
    }

    #[test]
    fn display1() {
        assert_eq!(Addrtype::Ip4.to_string(), "IP4");
    }
}
