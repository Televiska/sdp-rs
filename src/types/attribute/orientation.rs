use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Orientation {
    Portrait,
    Landscape,
    Seascape,
}

impl<'a> TryFrom<&'a str> for Orientation {
    type Error = crate::Error;

    fn try_from(from: &'a str) -> Result<Self, Self::Error> {
        match from {
            s if s.eq("portrait") => Ok(Self::Portrait),
            s if s.eq("landscape") => Ok(Self::Landscape),
            s if s.eq("seascape") => Ok(Self::Seascape),
            s => Err(crate::Error::parser(
                "orient attribute",
                format!("unknown value `{}`", s),
            )),
        }
    }
}

impl std::fmt::Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Portrait => write!(f, "portrait"),
            Self::Landscape => write!(f, "landscape"),
            Self::Seascape => write!(f, "seascape"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str1() {
        assert_eq!(Orientation::try_from("portrait"), Ok(Orientation::Portrait));
    }

    #[test]
    fn from_str2() {
        assert!(Orientation::try_from("Portrait").is_err());
    }

    #[test]
    fn display1() {
        assert_eq!(Orientation::Seascape.to_string(), "seascape");
    }
}
