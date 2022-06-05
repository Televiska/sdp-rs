use crate::{Attribute, Bandwidth, Connection, Error, Key, Media, SessionInformation};
use std::convert::{TryFrom, TryInto};

pub use crate::tokenizers::media_description::Tokenizer;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct MediaDescription {
    pub media: Media,
    pub info: Option<SessionInformation>,
    pub connections: Vec<Connection>,
    pub bandwidths: Vec<Bandwidth>,
    pub key: Option<Key>,
    pub attributes: Vec<Attribute>,
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
                .map(Into::into)
                .collect::<Vec<_>>(),
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
            media: crate::media::Tokenizer {
                media: "audio",
                port: "49170".into(),
                proto: "RTP/AVP",
                fmt: "0",
            },
            info: Some("audio media".into()),
            connections: vec![
                crate::connection::Tokenizer {
                    nettype: "IN",
                    addrtype: "IP4",
                    connection_address: "10.47.16.5".into(),
                },
                crate::connection::Tokenizer {
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
                media: Media {
                    media: crate::MediaType::Audio,
                    port: 49170,
                    num_of_ports: None,
                    proto: crate::ProtoType::RtpAvp,
                    fmt: "0".into()
                },
                info: Some(SessionInformation::new("audio media".into())),
                connections: vec![
                    Connection {
                        nettype: crate::Nettype::In,
                        addrtype: crate::Addrtype::Ip4,
                        connection_address: "10.47.16.5".parse::<IpAddr>().unwrap().into()
                    },
                    Connection {
                        nettype: crate::Nettype::In,
                        addrtype: crate::Addrtype::Ip4,
                        connection_address: "10.47.16.6".parse::<IpAddr>().unwrap().into()
                    },
                ],
                bandwidths: vec![
                    crate::Bandwidth {
                        bwtype: crate::Bwtype::Ct,
                        bandwidth: 1000
                    },
                    crate::Bandwidth {
                        bwtype: crate::Bwtype::As,
                        bandwidth: 551
                    }
                ],
                key: Some(Key {
                    method: crate::KeyMethod::Prompt,
                    encryption_key: Default::default()
                }),
                attributes: vec![
                    crate::Attribute::Rtpmap(crate::Rtpmap {
                        payload_type: 99,
                        encoding_name: "h232-199".into(),
                        clock_rate: 90000,
                        encoding_params: None
                    }),
                    crate::Attribute::Rtpmap(crate::Rtpmap {
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
            media: Media {
                media: crate::MediaType::Audio,
                port: 49170,
                num_of_ports: None,
                proto: crate::ProtoType::RtpAvp,
                fmt: "0".into(),
            },
            info: Some(SessionInformation::new("audio media".into())),
            connections: vec![
                Connection {
                    nettype: crate::Nettype::In,
                    addrtype: crate::Addrtype::Ip4,
                    connection_address: "10.47.16.5".parse::<IpAddr>().unwrap().into(),
                },
                Connection {
                    nettype: crate::Nettype::In,
                    addrtype: crate::Addrtype::Ip4,
                    connection_address: "10.47.16.6".parse::<IpAddr>().unwrap().into(),
                },
            ],
            bandwidths: vec![
                crate::Bandwidth {
                    bwtype: crate::Bwtype::Ct,
                    bandwidth: 1000,
                },
                crate::Bandwidth {
                    bwtype: crate::Bwtype::As,
                    bandwidth: 551,
                },
            ],
            key: Some(Key {
                method: crate::KeyMethod::Prompt,
                encryption_key: Default::default(),
            }),
            attributes: vec![
                crate::Attribute::Rtpmap(crate::Rtpmap {
                    payload_type: 99,
                    encoding_name: "h232-199".into(),
                    clock_rate: 90000,
                    encoding_params: None,
                }),
                crate::Attribute::Rtpmap(crate::Rtpmap {
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
            media: Media {
                media: crate::MediaType::Audio,
                port: 49170,
                num_of_ports: None,
                proto: crate::ProtoType::RtpAvp,
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
