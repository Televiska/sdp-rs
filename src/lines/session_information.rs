//! Types related to the session information line (`i=`).

/// The session information line (`i=`) tokenizer. This is low level stuff and you shouldn't
/// interact directly with it, unless you know what you are doing.
pub use crate::tokenizers::value::Tokenizer;

/// The session information line (`i=`) of SDP.
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub struct SessionInformation(String);

impl SessionInformation {
    pub fn new(session_information: String) -> Self {
        Self(session_information)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl From<SessionInformation> for String {
    fn from(session_information: SessionInformation) -> Self {
        session_information.0
    }
}

impl From<String> for SessionInformation {
    fn from(session_information: String) -> Self {
        Self(session_information)
    }
}

impl<'a> From<Tokenizer<'a, 'i'>> for SessionInformation {
    fn from(tokenizer: Tokenizer<'a, 'i'>) -> Self {
        Self(tokenizer.value.into())
    }
}

impl std::fmt::Display for SessionInformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "i={}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_tokenizer1() {
        let tokenizer: Tokenizer<'i'> = "a value".into();

        assert_eq!(
            SessionInformation::from(tokenizer),
            SessionInformation("a value".into())
        );
    }

    #[test]
    fn display1() {
        let session_info = SessionInformation::new("A simple session".into());

        assert_eq!(session_info.to_string(), "i=A simple session");
    }
}
