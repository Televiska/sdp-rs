use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ConferenceType {
    Broadcast,
    Meeting,
    Moderated,
    Test,
    H332,
}

impl<'a> TryFrom<&'a str> for ConferenceType {
    type Error = crate::Error;

    fn try_from(from: &'a str) -> Result<Self, Self::Error> {
        match from {
            s if s.eq("broadcast") => Ok(Self::Broadcast),
            s if s.eq("meeting") => Ok(Self::Meeting),
            s if s.eq("moderated") => Ok(Self::Moderated),
            s if s.eq("test") => Ok(Self::Test),
            s if s.eq("H332") => Ok(Self::H332),
            s => Err(crate::Error::parser(
                "type attribute",
                format!("unknown value `{}`", s),
            )),
        }
    }
}

impl std::fmt::Display for ConferenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Broadcast => write!(f, "broadcast"),
            Self::Meeting => write!(f, "meeting"),
            Self::Moderated => write!(f, "moderated"),
            Self::Test => write!(f, "test"),
            Self::H332 => write!(f, "H332"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str1() {
        assert_eq!(
            ConferenceType::try_from("broadcast"),
            Ok(ConferenceType::Broadcast)
        );
    }

    #[test]
    fn from_str2() {
        assert_eq!(ConferenceType::try_from("H332"), Ok(ConferenceType::H332));
    }

    #[test]
    fn from_str3() {
        assert!(ConferenceType::try_from("h332").is_err());
    }

    #[test]
    fn display1() {
        assert_eq!(ConferenceType::Moderated.to_string(), "moderated");
    }
}
