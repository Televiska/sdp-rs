use crate::tokenizers::value::Tokenizer;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Email(String);

impl Email {
    pub fn new(email: String) -> Self {
        Self(email)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}

impl From<String> for Email {
    fn from(email: String) -> Self {
        Self(email)
    }
}

impl<'a> From<Tokenizer<'a, 'e'>> for Email {
    fn from(tokenizer: Tokenizer<'a, 'e'>) -> Self {
        Self(tokenizer.value.into())
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "e={}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_tokenizer1() {
        let tokenizer: Tokenizer<'e'> = "hello@televiska.com".into();

        assert_eq!(Email::from(tokenizer), Email("hello@televiska.com".into()));
    }

    #[test]
    fn display1() {
        let email = Email::new("hello@televiska.com".into());

        assert_eq!(email.to_string(), "e=hello@televiska.com");
    }
}
