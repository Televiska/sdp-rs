pub use crate::tokenizers::value::Tokenizer;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub struct Uri(String);

impl Uri {
    pub fn new(uri: String) -> Self {
        Self(uri)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl From<Uri> for String {
    fn from(uri: Uri) -> Self {
        uri.0
    }
}

impl From<String> for Uri {
    fn from(uri: String) -> Self {
        Self(uri)
    }
}

impl<'a> From<Tokenizer<'a, 'u'>> for Uri {
    fn from(tokenizer: Tokenizer<'a, 'u'>) -> Self {
        Self(tokenizer.value.into())
    }
}

impl std::fmt::Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "u={}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_tokenizer1() {
        let tokenizer: Tokenizer<'u'> = "http://www.jdoe.example.com/home.html".into();

        assert_eq!(
            Uri::from(tokenizer),
            Uri("http://www.jdoe.example.com/home.html".into())
        );
    }

    #[test]
    fn display1() {
        let uri = Uri::new("https://televiska.com".into());

        assert_eq!(uri.to_string(), "u=https://televiska.com");
    }
}
