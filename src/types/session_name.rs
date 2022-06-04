use crate::tokenizers::value::Tokenizer;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SessionName(String);

impl SessionName {
    pub fn new(session_name: String) -> Self {
        Self(session_name)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl From<SessionName> for String {
    fn from(session_name: SessionName) -> Self {
        session_name.0
    }
}

impl From<String> for SessionName {
    fn from(session_name: String) -> Self {
        Self(session_name)
    }
}

impl<'a> From<Tokenizer<'a, 's'>> for SessionName {
    fn from(tokenizer: Tokenizer<'a, 's'>) -> Self {
        Self(tokenizer.value.into())
    }
}

impl std::fmt::Display for SessionName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "s={}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_tokenizer1() {
        let tokenizer: Tokenizer<'s'> = "a value".into();

        assert_eq!(SessionName::from(tokenizer), SessionName("a value".into()));
    }

    #[test]
    fn display1() {
        let session_name = SessionName::new("A simple session".into());

        assert_eq!(session_name.to_string(), "s=A simple session");
    }
}
