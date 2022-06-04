#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ProtoType {
    Udp,
    RtpAvp,
    RtpSavp,
    RtpSavpf,
    Other(String),
}

impl<'a> From<&'a str> for ProtoType {
    fn from(from: &'a str) -> Self {
        match from {
            s if s.eq("udp") => Self::Udp,
            s if s.eq("RTP/AVP") => Self::RtpAvp,
            s if s.eq("RTP/SAVP") => Self::RtpSavp,
            s if s.eq("RTP/SAVPF") => Self::RtpSavpf,
            s => Self::Other(s.into()),
        }
    }
}

impl std::fmt::Display for ProtoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Udp => write!(f, "udp"),
            Self::RtpAvp => write!(f, "RTP/AVP"),
            Self::RtpSavp => write!(f, "RTP/SAVP"),
            Self::RtpSavpf => write!(f, "RTP/SAVPF"),
            Self::Other(inner) => write!(f, "{}", inner),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str1() {
        assert_eq!(ProtoType::from("udp"), ProtoType::Udp);
    }

    #[test]
    fn from_str2() {
        assert_eq!(ProtoType::from("RTP/SAVPF"), ProtoType::RtpSavpf);
    }

    #[test]
    fn from_str3() {
        assert_eq!(
            ProtoType::from("rtp/savpf"),
            ProtoType::Other("rtp/savpf".into())
        );
    }

    #[test]
    fn display1() {
        assert_eq!(ProtoType::RtpSavpf.to_string(), "RTP/SAVPF");
    }

    #[test]
    fn display2() {
        assert_eq!(ProtoType::Udp.to_string(), "udp");
    }

    #[test]
    fn display3() {
        assert_eq!(
            ProtoType::Other("something".into()).to_string(),
            "something"
        );
    }
}
