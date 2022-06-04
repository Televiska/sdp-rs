#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Nettype {
    In,
    Other(String),
}

impl<'a> From<&'a str> for Nettype {
    fn from(s: &str) -> Self {
        if s.eq("IN") {
            Self::In
        } else {
            Self::Other(s.into())
        }
    }
}

impl std::fmt::Display for Nettype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::In => write!(f, "IN",),
            Self::Other(other) => write!(f, "{}", other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str1() {
        let part = "IN";

        assert_eq!(Nettype::from(part), Nettype::In);
    }

    #[test]
    fn from_str2() {
        let part = "in";

        assert_eq!(Nettype::from(part), Nettype::Other("in".into()));
    }

    #[test]
    fn display1() {
        assert_eq!(Nettype::In.to_string(), "IN");
    }
}
