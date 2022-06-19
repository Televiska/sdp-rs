use crate::{lines, Error};
use std::convert::{TryFrom, TryInto};

/// The Media description high level type tokenizer. It tokenizes all lines related to a Media
/// description.
/// This is low level stuff and you shouldn't interact directly
/// with it, unless you know what you are doing.
pub use crate::tokenizers::media_description::Tokenizer;

/// The Media description high level type. This type holds all types related to a complete Media
/// description (info, connections, bandwidths, attributes etc).
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct MediaDescription {
    pub media: lines::Media,
    pub info: Option<lines::SessionInformation>,
    pub connections: Vec<lines::Connection>,
    pub bandwidths: Vec<lines::Bandwidth>,
    pub key: Option<lines::Key>,
    pub attributes: Vec<lines::Attribute>,
}

impl<'a> TryFrom<Tokenizer<'a>> for MediaDescription {
    type Error = Error;

    fn try_from(tokenizer: Tokenizer<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            media: tokenizer.media.try_into()?,
            info: tokenizer.info.map(Into::into),
            connections: tokenizer
                .connections
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            bandwidths: tokenizer
                .bandwidths
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            key: tokenizer.key.map(Into::into),
            attributes: tokenizer
                .attributes
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl std::fmt::Display for MediaDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "m={}", self.media)?;
        if let Some(info) = &self.info {
            write!(f, "\r\n{}", info)?
        }
        for connection in self.connections.iter() {
            write!(f, "\r\n{}", connection)?
        }
        for bandwidth in self.bandwidths.iter() {
            write!(f, "\r\n{}", bandwidth)?
        }
        if let Some(key) = &self.key {
            write!(f, "\r\n{}", key)?
        }
        for attribute in self.attributes.iter() {
            write!(f, "\r\n{}", attribute)?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::IpAddr;

    #[test]
    fn from_tokenizer1() {
        let tokenizer = Tokenizer {
            media: lines::media::Tokenizer {
                media: "audio",
                port: "49170".into(),
                proto: "RTP/AVP",
                fmt: "0",
            },
            info: Some("audio media".into()),
            connections: vec![
                lines::connection::Tokenizer {
                    nettype: "IN",
                    addrtype: "IP4",
                    connection_address: "10.47.16.5".into(),
                },
                lines::connection::Tokenizer {
                    nettype: "IN",
                    addrtype: "IP4",
                    connection_address: "10.47.16.6".into(),
                },
            ],
            bandwidths: vec![("CT", "1000").into(), ("AS", "551").into()],
            key: Some("prompt".into()),
            attributes: vec![
                ("rtpmap", "99 h232-199/90000").into(),
                ("rtpmap", "90 h263-1998/90000").into(),
            ],
        };

        assert_eq!(
            MediaDescription::try_from(tokenizer),
            Ok(MediaDescription {
                media: lines::Media {
                    media: lines::media::MediaType::Audio,
                    port: 49170,
                    num_of_ports: None,
                    proto: lines::media::ProtoType::RtpAvp,
                    fmt: "0".into()
                },
                info: Some(lines::SessionInformation::new("audio media".into())),
                connections: vec![
                    lines::Connection {
                        nettype: lines::common::Nettype::In,
                        addrtype: lines::common::Addrtype::Ip4,
                        connection_address: "10.47.16.5".parse::<IpAddr>().unwrap().into()
                    },
                    lines::Connection {
                        nettype: lines::common::Nettype::In,
                        addrtype: lines::common::Addrtype::Ip4,
                        connection_address: "10.47.16.6".parse::<IpAddr>().unwrap().into()
                    },
                ],
                bandwidths: vec![
                    lines::Bandwidth {
                        bwtype: lines::bandwidth::Bwtype::Ct,
                        bandwidth: 1000
                    },
                    lines::Bandwidth {
                        bwtype: lines::bandwidth::Bwtype::As,
                        bandwidth: 551
                    }
                ],
                key: Some(lines::Key {
                    method: lines::key::KeyMethod::Prompt,
                    encryption_key: Default::default()
                }),
                attributes: vec![
                    lines::Attribute::Rtpmap(lines::attribute::Rtpmap {
                        payload_type: 99,
                        encoding_name: "h232-199".into(),
                        clock_rate: 90000,
                        encoding_params: None
                    }),
                    lines::Attribute::Rtpmap(lines::attribute::Rtpmap {
                        payload_type: 90,
                        encoding_name: "h263-1998".into(),
                        clock_rate: 90000,
                        encoding_params: None
                    }),
                ]
            })
        );
    }

    #[test]
    fn display1() {
        let media_description = MediaDescription {
            media: lines::Media {
                media: lines::media::MediaType::Audio,
                port: 49170,
                num_of_ports: None,
                proto: lines::media::ProtoType::RtpAvp,
                fmt: "0".into(),
            },
            info: Some(lines::SessionInformation::new("audio media".into())),
            connections: vec![
                lines::Connection {
                    nettype: lines::common::Nettype::In,
                    addrtype: lines::common::Addrtype::Ip4,
                    connection_address: "10.47.16.5".parse::<IpAddr>().unwrap().into(),
                },
                lines::Connection {
                    nettype: lines::common::Nettype::In,
                    addrtype: lines::common::Addrtype::Ip4,
                    connection_address: "10.47.16.6".parse::<IpAddr>().unwrap().into(),
                },
            ],
            bandwidths: vec![
                lines::Bandwidth {
                    bwtype: lines::bandwidth::Bwtype::Ct,
                    bandwidth: 1000,
                },
                lines::Bandwidth {
                    bwtype: lines::bandwidth::Bwtype::As,
                    bandwidth: 551,
                },
            ],
            key: Some(lines::Key {
                method: lines::key::KeyMethod::Prompt,
                encryption_key: Default::default(),
            }),
            attributes: vec![
                lines::Attribute::Rtpmap(lines::attribute::Rtpmap {
                    payload_type: 99,
                    encoding_name: "h232-199".into(),
                    clock_rate: 90000,
                    encoding_params: None,
                }),
                lines::Attribute::Rtpmap(lines::attribute::Rtpmap {
                    payload_type: 90,
                    encoding_name: "h263-1998".into(),
                    clock_rate: 90000,
                    encoding_params: None,
                }),
            ],
        };

        assert_eq!(
            media_description.to_string(),
            concat!(
                "m=audio 49170 RTP/AVP 0\r\n",
                "i=audio media\r\n",
                "c=IN IP4 10.47.16.5\r\n",
                "c=IN IP4 10.47.16.6\r\n",
                "b=CT:1000\r\n",
                "b=AS:551\r\n",
                "k=prompt\r\n",
                "a=rtpmap:99 h232-199/90000\r\n",
                "a=rtpmap:90 h263-1998/90000"
            )
        );
    }

    #[test]
    fn display2() {
        let media_description = MediaDescription {
            media: lines::Media {
                media: lines::media::MediaType::Audio,
                port: 49170,
                num_of_ports: None,
                proto: lines::media::ProtoType::RtpAvp,
                fmt: "0".into(),
            },
            info: None,
            connections: vec![],
            bandwidths: vec![],
            key: None,
            attributes: vec![],
        };

        assert_eq!(
            media_description.to_string(),
            concat!("m=audio 49170 RTP/AVP 0",)
        );
    }
}
