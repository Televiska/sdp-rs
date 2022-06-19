/// The Bwtype as it appears in the bandwidth (`b=`) line. It's not a `Copy` type since it
/// supports abstract types, not even defined in any RFC.
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub enum Bwtype {
    Ct,
    As,
    Other(String),
}

impl<'a> From<&'a str> for Bwtype {
    fn from(s: &str) -> Self {
        match s {
            s if s.eq("CT") => Self::Ct,
            s if s.eq("AS") => Self::As,
            _ => Self::Other(s.into()),
        }
    }
}

impl std::fmt::Display for Bwtype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ct => write!(f, "CT",),
            Self::As => write!(f, "AS",),
            Self::Other(other) => write!(f, "{}", other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str1() {
        let part = "CT";

        assert_eq!(Bwtype::from(part), Bwtype::Ct);
    }

    #[test]
    fn from_str2() {
        let part = "AS";

        assert_eq!(Bwtype::from(part), Bwtype::As);
    }

    #[test]
    fn from_str3() {
        let part = "As";

        assert_eq!(Bwtype::from(part), Bwtype::Other("As".into()));
    }

    #[test]
    fn display1() {
        assert_eq!(Bwtype::As.to_string(), "AS");
    }
}
