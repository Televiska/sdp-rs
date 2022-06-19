//! Types related to the attribute line (`a=`).
//!
//! If a (common) attribute that has clear syntax
//! (either through an ABNF syntax or through something else) is missing from here, feel free to
//! open an issue. With nom it should be easy to create a typed form of it instead of dealing with
//! the `Attribute::Other` variant.

mod conference_type;
mod fmtp;
mod orientation;
mod rtpmap;

pub use conference_type::ConferenceType;
pub use fmtp::Fmtp;
pub use orientation::Orientation;
pub use rtpmap::Rtpmap;

/// The attribute line (`a=`) tokenizer. This is low level stuff and you shouldn't interact directly
/// with it, unless you know what you are doing.
pub use crate::tokenizers::key_optvalue::Tokenizer;
use std::convert::TryFrom;

/// foo bar
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Attribute {
    Cat(String),
    Keywds(String),
    Tool(String),
    Ptime(f32),
    Maxptime(f32),
    Rtpmap(Rtpmap),
    Recvonly,
    Sendrecv,
    Sendonly,
    Inactive,
    Orient(Orientation),
    Type(ConferenceType),
    Charset(String),
    Sdplang(String),
    Lang(String),
    Framerate(f32),
    Quality(i32),
    Other(String, Option<String>),
}

//TODO: add warning log on errors behind a feature flag
impl<'a> TryFrom<Tokenizer<'a, 'a'>> for Attribute {
    type Error = crate::Error;

    fn try_from(tokenizer: Tokenizer<'a, 'a'>) -> Result<Self, Self::Error> {
        Ok(match (tokenizer.key, tokenizer.value) {
            (key, Some(value)) if key.eq("cat") => Self::Cat(value.into()),
            (key, Some(value)) if key.eq("keywds") => Self::Keywds(value.into()),
            (key, Some(value)) if key.eq("tool") => Self::Tool(value.into()),
            (key, value) if key.eq("ptime") => match value {
                Some(value) => value
                    .parse()
                    .map(Self::Ptime)
                    .map_err(|e| crate::Error::parser_with_error("ptime attribute", value, e))?,
                None => {
                    return Err(crate::Error::parser_with_error(
                        "ptime attribute",
                        "",
                        "missing value",
                    ))
                }
            },
            (key, value) if key.eq("maxptime") => match value {
                Some(value) => value
                    .parse()
                    .map(Self::Maxptime)
                    .map_err(|e| crate::Error::parser_with_error("maxptime attribute", value, e))?,
                None => {
                    return Err(crate::Error::parser_with_error(
                        "maxptime attribute",
                        "",
                        "missing value",
                    ))
                }
            },
            (key, value) if key.eq("rtpmap") => {
                match value {
                    Some(value) => Self::Rtpmap(Rtpmap::try_from(value).map_err(|e| {
                        crate::Error::parser_with_error("rtpmap attribute", value, e)
                    })?),
                    None => {
                        return Err(crate::Error::parser_with_error(
                            "rtpmap attribute",
                            "",
                            "missing value",
                        ))
                    }
                }
            }
            (key, None) if key.eq("recvonly") => Self::Recvonly,
            (key, None) if key.eq("sendrecv") => Self::Sendrecv,
            (key, None) if key.eq("sendonly") => Self::Sendonly,
            (key, None) if key.eq("inacive") => Self::Inactive,
            (key, value) if key.eq("orient") => match value {
                Some(value) => Orientation::try_from(value)
                    .map(Self::Orient)
                    .map_err(|e| crate::Error::parser_with_error("orient attribute", value, e))?,
                None => {
                    return Err(crate::Error::parser_with_error(
                        "orient attribute",
                        "",
                        "missing value",
                    ))
                }
            },
            (key, value) if key.eq("type") => match value {
                Some(value) => ConferenceType::try_from(value)
                    .map(Self::Type)
                    .map_err(|e| crate::Error::parser_with_error("type attribute", value, e))?,
                None => return Err(crate::Error::parser_with_error("type", "", "missing value")),
            },
            (key, Some(value)) if key.eq("charset") => Self::Charset(value.into()),
            (key, Some(value)) if key.eq("sdplang") => Self::Sdplang(value.into()),
            (key, Some(value)) if key.eq("lang") => Self::Lang(value.into()),
            (key, value) if key.eq("framerate") => match value {
                Some(value) => value.parse().map(Self::Framerate).map_err(|e| {
                    crate::Error::parser_with_error("framerate attribute", value, e)
                })?,
                None => {
                    return Err(crate::Error::parser_with_error(
                        "framerate attribute",
                        "",
                        "missing value",
                    ))
                }
            },
            (key, value) if key.eq("quality") => match value {
                Some(value) => value
                    .parse()
                    .map(Self::Quality)
                    .map_err(|e| crate::Error::parser_with_error("quality attribute", value, e))?,
                None => {
                    return Err(crate::Error::parser_with_error(
                        "quality attribute",
                        "",
                        "missing value",
                    ))
                }
            },
            (key, value) => Self::Other(key.into(), value.map(Into::into)),
        })
    }
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cat(cat) => write!(f, "a=cat:{}", cat),
            Self::Keywds(keywds) => write!(f, "a=keywds:{}", keywds),
            Self::Tool(tool) => write!(f, "a=tool:{}", tool),
            Self::Ptime(ptime) => write!(f, "a=ptime:{}", ptime),
            Self::Maxptime(maxptime) => write!(f, "a=maxptime:{}", maxptime),
            Self::Rtpmap(rtpmap) => write!(f, "a=rtpmap:{}", rtpmap),
            Self::Recvonly => write!(f, "a=recvonly"),
            Self::Sendrecv => write!(f, "a=sendrecv"),
            Self::Sendonly => write!(f, "a=sendonly"),
            Self::Inactive => write!(f, "a=inactive"),
            Self::Orient(orientation) => write!(f, "a=orient:{}", orientation),
            Self::Type(conference_type) => write!(f, "a=type:{}", conference_type),
            Self::Charset(charset) => write!(f, "a=charset:{}", charset),
            Self::Sdplang(sdplang) => write!(f, "a=sdplang:{}", sdplang),
            Self::Lang(lang) => write!(f, "a=lang:{}", lang),
            Self::Framerate(framerate) => write!(f, "a=framerate:{}", framerate),
            Self::Quality(quality) => write!(f, "a=quality:{}", quality),
            Self::Other(key, Some(value)) => write!(f, "a={}:{}", key, value),
            Self::Other(key, None) => write!(f, "a={}", key),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_tokenizer1() {
        let tokenizer: Tokenizer<'a'> = ("key", Some("something")).into();

        assert_eq!(
            Attribute::try_from(tokenizer).unwrap(),
            Attribute::Other("key".into(), Some("something".into()))
        );
    }

    #[test]
    fn from_tokenizer2() {
        let tokenizer: Tokenizer<'a'> = ("orient", None).into();

        assert!(Attribute::try_from(tokenizer).is_err(),);
    }

    #[test]
    fn from_tokenizer3() {
        let tokenizer: Tokenizer<'a'> = ("orient", Some("Portrait")).into();

        assert!(Attribute::try_from(tokenizer).is_err(),);
    }

    #[test]
    fn from_tokenizer4() {
        let tokenizer: Tokenizer<'a'> = ("orient", Some("portrait")).into();

        assert_eq!(
            Attribute::try_from(tokenizer).unwrap(),
            Attribute::Orient(Orientation::Portrait)
        );
    }

    #[test]
    fn display1() {
        assert_eq!(
            Attribute::Cat("foo.bar".to_string()).to_string(),
            "a=cat:foo.bar"
        );
    }

    #[test]
    fn display2() {
        assert_eq!(Attribute::Ptime(20.0).to_string(), "a=ptime:20");
    }

    #[test]
    fn display3() {
        assert_eq!(
            Attribute::Orient(Orientation::Portrait).to_string(),
            "a=orient:portrait"
        );
    }

    #[test]
    fn display4() {
        assert_eq!(Attribute::Recvonly.to_string(), "a=recvonly");
    }

    #[test]
    fn display5() {
        assert_eq!(
            Attribute::Other("foo".into(), Some("bar".into())).to_string(),
            "a=foo:bar"
        );
    }

    #[test]
    fn display6() {
        assert_eq!(Attribute::Other("foo".into(), None).to_string(), "a=foo");
    }
}
