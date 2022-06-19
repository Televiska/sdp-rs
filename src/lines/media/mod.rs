//! Types related to the media line (`m=`).

mod media_type;
mod proto_type;

/// The media line (`m=`) tokenizer. This is low level stuff and you shouldn't interact directly
/// with it, unless you know what you are doing.
pub use crate::tokenizers::media::Tokenizer;
pub use media_type::MediaType;
pub use proto_type::ProtoType;

use crate::Error;
use std::convert::TryFrom;

/// A media line (`m=`) of SDP. Media is part of the high level [crate::MediaDescription] type.
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub struct Media {
    pub media: MediaType,
    pub port: u16,
    pub num_of_ports: Option<u8>,
    pub proto: ProtoType,
    pub fmt: String,
}

impl<'a> TryFrom<Tokenizer<'a>> for Media {
    type Error = Error;

    fn try_from(tokenizer: Tokenizer<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            media: tokenizer.media.into(),
            port: tokenizer.port.port.parse().map_err(|e| {
                Self::Error::parser_with_error("media port", tokenizer.port.port, e)
            })?,
            num_of_ports: tokenizer
                .port
                .num_of_ports
                .map(|num| {
                    num.parse()
                        .map_err(|e| Self::Error::parser_with_error("media num of ports", num, e))
                })
                .transpose()?,
            proto: tokenizer.proto.into(),
            fmt: tokenizer.fmt.into(),
        })
    }
}

impl std::fmt::Display for Media {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.num_of_ports {
            Some(num_of_ports) => write!(
                f,
                "{} {}/{} {} {}",
                self.media, self.port, num_of_ports, self.proto, self.fmt
            ),
            None => write!(
                f,
                "{} {} {} {}",
                self.media, self.port, self.proto, self.fmt
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_tokenizer1() {
        let tokenizer = Tokenizer {
            media: "audio",
            port: ("49170", "2").into(),
            proto: "RTP/AVP",
            fmt: "0",
        };

        assert_eq!(
            Media::try_from(tokenizer),
            Ok(Media {
                media: MediaType::Audio,
                port: 49170,
                num_of_ports: Some(2),
                proto: ProtoType::RtpAvp,
                fmt: "0".into(),
            })
        );
    }

    #[test]
    fn display1() {
        assert_eq!(
            Media {
                media: MediaType::Audio,
                port: 49170,
                num_of_ports: Some(2),
                proto: ProtoType::RtpAvp,
                fmt: "0".into(),
            }
            .to_string(),
            "audio 49170/2 RTP/AVP 0"
        );
    }
}
