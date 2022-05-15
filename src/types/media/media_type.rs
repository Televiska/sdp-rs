#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MediaType {
    Audio,
    Video,
    Text,
    Application,
    Message,
    Image,
    Other(String),
}

impl<'a> From<&'a str> for MediaType {
    fn from(from: &'a str) -> Self {
        match from {
            s if s.eq("audio") => Self::Audio,
            s if s.eq("video") => Self::Video,
            s if s.eq("text") => Self::Text,
            s if s.eq("application") => Self::Application,
            s if s.eq("message") => Self::Message,
            s if s.eq("image") => Self::Image,
            s => Self::Other(s.into()),
        }
    }
}

impl std::fmt::Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Audio => write!(f, "audio"),
            Self::Video => write!(f, "video"),
            Self::Text => write!(f, "text"),
            Self::Application => write!(f, "application"),
            Self::Message => write!(f, "message"),
            Self::Image => write!(f, "image"),
            Self::Other(inner) => write!(f, "{}", inner),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str1() {
        assert_eq!(MediaType::from("audio"), MediaType::Audio);
    }

    #[test]
    fn from_str2() {
        assert_eq!(MediaType::from("Video"), MediaType::Other("Video".into()));
    }

    #[test]
    fn display1() {
        assert_eq!(MediaType::Audio.to_string(), "audio");
    }

    #[test]
    fn display2() {
        assert_eq!(MediaType::Image.to_string(), "image");
    }

    #[test]
    fn display3() {
        assert_eq!(
            MediaType::Other("something".into()).to_string(),
            "something"
        );
    }
}
