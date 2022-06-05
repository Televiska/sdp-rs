mod key_method;

pub use crate::tokenizers::key_optvalue::Tokenizer;
pub use key_method::KeyMethod;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub struct Key {
    pub method: KeyMethod,
    pub encryption_key: String,
}

impl<'a> From<Tokenizer<'a, 'k'>> for Key {
    fn from(tokenizer: Tokenizer<'a, 'k'>) -> Self {
        Self {
            method: tokenizer.key.into(),
            encryption_key: tokenizer.value.map(Into::into).unwrap_or_default(),
        }
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.method {
            KeyMethod::Prompt => write!(f, "k={}", self.method),
            _ => write!(f, "k={}:{}", self.method, self.encryption_key),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_tokenizer1() {
        let tokenizer: Tokenizer<'k'> = ("key", Some("something")).into();

        assert_eq!(
            Key::from(tokenizer),
            Key {
                method: KeyMethod::Other("key".into()),
                encryption_key: "something".into()
            }
        );
    }

    #[test]
    fn from_tokenizer2() {
        let tokenizer: Tokenizer<'k'> = ("clear", Some("something")).into();

        assert_eq!(
            Key::from(tokenizer),
            Key {
                method: KeyMethod::Clear,
                encryption_key: "something".into()
            }
        );
    }

    #[test]
    fn from_tokenizer3() {
        let tokenizer: Tokenizer<'k'> = ("prompt", None).into();

        assert_eq!(
            Key::from(tokenizer),
            Key {
                method: KeyMethod::Prompt,
                encryption_key: "".into()
            }
        );
    }

    #[test]
    fn display1() {
        let key = Key {
            method: KeyMethod::Clear,
            encryption_key: "password".into(),
        };

        assert_eq!(key.to_string(), "k=clear:password");
    }

    #[test]
    fn display2() {
        let key = Key {
            method: KeyMethod::Prompt,
            encryption_key: "".into(),
        };

        assert_eq!(key.to_string(), "k=prompt");
    }
}
