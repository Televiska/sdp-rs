#[derive(Debug, PartialEq, Eq, Clone)]
pub enum KeyMethod {
    Clear,
    Base64,
    Uri,
    Prompt,
    Other(String),
}

impl<'a> From<&'a str> for KeyMethod {
    fn from(s: &str) -> Self {
        match s {
            s if s.eq("clear") => Self::Clear,
            s if s.eq("base64") => Self::Base64,
            s if s.eq("uri") => Self::Uri,
            s if s.eq("prompt") => Self::Prompt,
            _ => Self::Other(s.into()),
        }
    }
}

impl std::fmt::Display for KeyMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Clear => write!(f, "clear"),
            Self::Base64 => write!(f, "base64"),
            Self::Uri => write!(f, "uri"),
            Self::Prompt => write!(f, "prompt"),
            Self::Other(other) => write!(f, "{}", other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str1() {
        assert_eq!(KeyMethod::from("clear"), KeyMethod::Clear);
    }

    #[test]
    fn display1() {
        assert_eq!(KeyMethod::Base64.to_string(), "base64");
    }
}
