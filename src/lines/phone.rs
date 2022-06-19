//! Types related to the phone line (`p=`).

/// The phone line (`p=`) tokenizer. This is low level stuff and you shouldn't interact directly
/// with it, unless you know what you are doing.
pub use crate::tokenizers::value::Tokenizer;

/// A phone number line (`p=`) of SDP. Note that more than one such line could exist in an SDP
/// message, that's why [crate::SessionDescription] has a `Vec<Phone>` defined.
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub struct Phone(String);

impl Phone {
    pub fn new(phone: String) -> Self {
        Self(phone)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl From<Phone> for String {
    fn from(phone: Phone) -> Self {
        phone.0
    }
}

impl From<String> for Phone {
    fn from(phone: String) -> Self {
        Self(phone)
    }
}

impl<'a> From<Tokenizer<'a, 'p'>> for Phone {
    fn from(tokenizer: Tokenizer<'a, 'p'>) -> Self {
        Self(tokenizer.value.into())
    }
}

impl std::fmt::Display for Phone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "p={}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_tokenizer1() {
        let tokenizer: Tokenizer<'p'> = "+1234567890".into();

        assert_eq!(Phone::from(tokenizer), Phone("+1234567890".into()));
    }

    #[test]
    fn display1() {
        let phone = Phone::new("+1234567890".into());

        assert_eq!(phone.to_string(), "p=+1234567890");
    }
}
