use crate::{tokenizers::attributes::name_optvalue::Tokenizer as ParamsTokenizer, Error};

pub use crate::tokenizers::attributes::fmtp::Tokenizer;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub struct Fmtp {
    pub fmt: String,
    pub params: String,
}

impl Fmtp {
    pub fn params_list(&self) -> Result<Vec<(&str, Option<&str>)>, Error> {
        use nom::multi::many1;

        let (_, tokenizers) = many1(ParamsTokenizer::tokenize)(&self.params)?;

        Ok(tokenizers.into_iter().map(|t| (t.name, t.value)).collect())
    }
}

impl<'a> From<Tokenizer<'a>> for Fmtp {
    fn from(tokenizer: Tokenizer<'a>) -> Self {
        Self {
            fmt: tokenizer.fmt.into(),
            params: tokenizer.params.into(),
        }
    }
}

impl std::fmt::Display for Fmtp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.fmt, self.params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn from_str1() {
        let tokenizer = Tokenizer {
            fmt: "96",
            params: "profile-level-id=42e016;max-mbps=108000;max-fs=3600",
        };

        assert_eq!(
            Fmtp::try_from(tokenizer),
            Ok(Fmtp {
                fmt: "96".into(),
                params: "profile-level-id=42e016;max-mbps=108000;max-fs=3600".into(),
            })
        );
    }

    #[test]
    fn params_list1() {
        let fmtp = Fmtp {
            fmt: "96".into(),
            params: "profile-level-id=42e016;max-mbps=108000;max-fs=3600".into(),
        };

        assert_eq!(
            fmtp.params_list(),
            Ok(vec![
                ("profile-level-id", Some("42e016")),
                ("max-mbps", Some("108000")),
                ("max-fs", Some("3600")),
            ])
        );
    }

    #[test]
    fn params_list2() {
        let fmtp = Fmtp {
            fmt: "96".into(),
            params: "profile-level-id=42e016;max-mbps=108000;max-fs=3600;".into(),
        };

        assert_eq!(
            fmtp.params_list(),
            Ok(vec![
                ("profile-level-id", Some("42e016")),
                ("max-mbps", Some("108000")),
                ("max-fs", Some("3600")),
            ])
        );
    }

    #[test]
    fn params_list3() {
        let fmtp = Fmtp {
            fmt: "96".into(),
            params: "profile-level-id;max-mbps=108000;max-fs".into(),
        };

        assert_eq!(
            fmtp.params_list(),
            Ok(vec![
                ("profile-level-id", None),
                ("max-mbps", Some("108000")),
                ("max-fs", None),
            ])
        );
    }

    #[test]
    fn display1() {
        assert_eq!(
            Fmtp {
                fmt: "96".into(),
                params: "profile-level-id;max-mbps=108000;max-fs".into(),
            }
            .to_string(),
            "96 profile-level-id;max-mbps=108000;max-fs"
        );
    }
}
