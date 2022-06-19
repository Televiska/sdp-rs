use crate::{lines, Error};
use std::convert::{TryFrom, TryInto};
use vec1::Vec1;

/// The Session description high level type tokenizer. It tokenizes an SDP message. This is low
/// level stuff and you shouldn't interact directly with it, unless you know what you are doing.
pub use crate::tokenizers::session_description::Tokenizer;

/// The Session description. This is the main `sdp-rs` type that describes the SDP message.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct SessionDescription {
    pub version: lines::Version,
    pub origin: lines::Origin,
    pub session_name: lines::SessionName,
    pub session_info: Option<lines::SessionInformation>,
    pub uri: Option<lines::Uri>,
    pub emails: Vec<lines::Email>,
    pub phones: Vec<lines::Phone>,
    pub connection: Option<lines::Connection>,
    pub bandwidths: Vec<lines::Bandwidth>,
    pub times: Vec1<crate::Time>,
    pub key: Option<lines::Key>,
    pub attributes: Vec<lines::Attribute>,
    pub media_descriptions: Vec<crate::MediaDescription>,
}

impl TryFrom<String> for SessionDescription {
    type Error = Error;

    fn try_from(from: String) -> Result<Self, Self::Error> {
        let tokenizer = Tokenizer::tokenize(&from)?;
        Self::try_from(tokenizer.1)
    }
}

impl TryFrom<&str> for SessionDescription {
    type Error = Error;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        let tokenizer = Tokenizer::tokenize(from)?;
        Self::try_from(tokenizer.1)
    }
}

impl std::str::FromStr for SessionDescription {
    type Err = Error;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        Self::try_from(str)
    }
}

impl<'a> TryFrom<Tokenizer<'a>> for SessionDescription {
    type Error = Error;

    fn try_from(tokenizer: Tokenizer<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            version: tokenizer.version.try_into()?,
            origin: tokenizer.origin.try_into()?,
            session_name: tokenizer.session_name.into(),
            session_info: tokenizer.session_info.map(Into::into),
            uri: tokenizer.uri.map(Into::into),
            emails: tokenizer
                .emails
                .into_iter()
                .map(Into::into)
                .collect::<Vec<_>>(),
            phones: tokenizer
                .phones
                .into_iter()
                .map(Into::into)
                .collect::<Vec<_>>(),
            connection: tokenizer.connection.map(TryInto::try_into).transpose()?,
            bandwidths: tokenizer
                .bandwidths
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            times: tokenizer
                .times
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?
                .try_into()
                .map_err(|_| Error::parser("times", "missing time(s) line(s)"))?,
            key: tokenizer.key.map(Into::into),
            attributes: tokenizer
                .attributes
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            media_descriptions: tokenizer
                .media_descriptions
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl std::fmt::Display for SessionDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\r\n", self.version)?;
        write!(f, "{}\r\n", self.origin)?;
        write!(f, "{}\r\n", self.session_name)?;
        if let Some(info) = &self.session_info {
            write!(f, "{}\r\n", info)?;
        }
        if let Some(uri) = &self.uri {
            write!(f, "{}\r\n", uri)?;
        }
        for email in self.emails.iter() {
            write!(f, "{}\r\n", email)?;
        }
        for phone in self.phones.iter() {
            write!(f, "{}\r\n", phone)?;
        }
        if let Some(connection) = &self.connection {
            write!(f, "{}\r\n", connection)?;
        }
        for bandwidth in self.bandwidths.iter() {
            write!(f, "{}\r\n", bandwidth)?;
        }
        for time in self.times.iter() {
            write!(f, "{}\r\n", time)?;
        }
        if let Some(key) = &self.key {
            write!(f, "{}\r\n", key)?;
        }
        for attribute in self.attributes.iter() {
            write!(f, "{}\r\n", attribute)?;
        }
        for media_description in self.media_descriptions.iter() {
            write!(f, "{}\r\n", media_description)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use std::net::{IpAddr, Ipv4Addr};
    use vec1::vec1;

    #[test]
    fn from_tokenizer1() {
        let tokenizer = Tokenizer {
            version: "0".into(),
            origin: crate::tokenizers::origin::Tokenizer {
                username: "Alice",
                sess_id: "2890844526",
                sess_version: "2890842807",
                nettype: "IN",
                addrtype: "IP4",
                unicast_address: "10.47.16.5",
            },
            session_name: "-".into(),
            session_info: Some("A Seminar on the session description protocol".into()),
            uri: Some("http://www.example.com/seminars/sdp.pdf".into()),
            emails: vec!["alice@example.com (Alice Smith)".into()],
            phones: vec!["+1 911-345-1160".into()],
            connection: Some(crate::tokenizers::connection::Tokenizer {
                nettype: "IN",
                addrtype: "IP4",
                connection_address: ("10.47.16.5").into(),
            }),
            bandwidths: vec![("CT", "1024").into()],
            times: vec1![crate::time::Tokenizer {
                active: ("2854678930", "2854679000").into(),
                repeat: vec![("604800", "3600", vec!["0", "90000"]).into(),],
                zone: Some(vec![("2882844526", "-1h"), ("2898848070", "0h")].into())
            }],
            key: Some(("clear", "password").into()),
            attributes: vec!["recvonly".into()],
            media_descriptions: vec![crate::tokenizers::media_description::Tokenizer {
                media: crate::tokenizers::media::Tokenizer {
                    media: "audio",
                    port: "49170".into(),
                    proto: "RTP/AVP",
                    fmt: "0",
                },
                info: Some("audio media".into()),
                connections: vec![
                    crate::tokenizers::connection::Tokenizer {
                        nettype: "IN",
                        addrtype: "IP4",
                        connection_address: "10.47.16.5".into(),
                    },
                    crate::tokenizers::connection::Tokenizer {
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
            }],
        };

        let expected_times = vec1![crate::Time {
            active: lines::Active {
                start: 2854678930,
                stop: 2854679000,
            },
            repeat: vec![lines::Repeat {
                interval: lines::common::TypedTime::None(Duration::seconds(604800)),
                duration: lines::common::TypedTime::None(Duration::seconds(3600)),
                offsets: vec![
                    lines::common::TypedTime::None(Duration::seconds(0)),
                    lines::common::TypedTime::None(Duration::seconds(90000))
                ],
            },],
            zone: Some(lines::Zone {
                parts: vec![
                    lines::zone::ZonePart {
                        adjustment_time: 2882844526,
                        offset: lines::common::TypedTime::Hours(Duration::hours(-1)),
                    },
                    lines::zone::ZonePart {
                        adjustment_time: 2898848070,
                        offset: lines::common::TypedTime::Hours(Duration::hours(0)),
                    },
                ],
            })
        }];

        let expected_media_description = crate::MediaDescription {
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
            SessionDescription::try_from(tokenizer),
            Ok(SessionDescription {
                version: lines::Version::V0,
                origin: lines::Origin {
                    username: "Alice".into(),
                    sess_id: "2890844526".into(),
                    sess_version: "2890842807".into(),
                    nettype: lines::common::Nettype::In,
                    addrtype: lines::common::Addrtype::Ip4,
                    unicast_address: IpAddr::V4(Ipv4Addr::new(10, 47, 16, 5)),
                },
                session_name: lines::SessionName::new("-".into()),
                session_info: Some(lines::SessionInformation::new(
                    "A Seminar on the session description protocol".into()
                )),
                uri: Some(lines::Uri::new(
                    "http://www.example.com/seminars/sdp.pdf".into()
                )),
                emails: vec![lines::Email::new("alice@example.com (Alice Smith)".into())],
                phones: vec![lines::Phone::new("+1 911-345-1160".into())],
                connection: Some(lines::Connection {
                    nettype: lines::common::Nettype::In,
                    addrtype: lines::common::Addrtype::Ip4,
                    connection_address: lines::connection::ConnectionAddress {
                        base: "10.47.16.5".parse().unwrap(),
                        ttl: None,
                        numaddr: None
                    }
                }),
                bandwidths: vec![lines::Bandwidth {
                    bwtype: lines::bandwidth::Bwtype::Ct,
                    bandwidth: 1024,
                }],
                times: expected_times,
                key: Some(lines::Key {
                    method: lines::key::KeyMethod::Clear,
                    encryption_key: "password".into()
                }),
                attributes: vec![lines::Attribute::Recvonly],
                media_descriptions: vec![expected_media_description]
            })
        );
    }

    #[test]
    fn from_tokenizer2() {
        let tokenizer = Tokenizer {
            version: "0".into(),
            origin: crate::tokenizers::origin::Tokenizer {
                username: "Alice",
                sess_id: "2890844526",
                sess_version: "2890842807",
                nettype: "IN",
                addrtype: "IP4",
                unicast_address: "10.47.16.5",
            },
            session_name: "-".into(),
            session_info: None,
            uri: None,
            emails: vec![],
            phones: vec![],
            connection: None,
            bandwidths: vec![],
            times: vec1![crate::tokenizers::time::Tokenizer {
                active: ("2854678930", "2854679000").into(),
                repeat: vec![],
                zone: None
            }],
            key: None,
            attributes: vec![],
            media_descriptions: vec![crate::media_description::Tokenizer {
                media: crate::tokenizers::media::Tokenizer {
                    media: "audio",
                    port: "49170".into(),
                    proto: "RTP/AVP",
                    fmt: "0",
                },
                info: None,
                connections: vec![crate::tokenizers::connection::Tokenizer {
                    nettype: "IN",
                    addrtype: "IP4",
                    connection_address: "10.47.16.6".into(),
                }],
                bandwidths: vec![("AS", "551").into()],
                key: None,
                attributes: vec![],
            }],
        };

        let expected_times = vec1![crate::Time {
            active: lines::Active {
                start: 2854678930,
                stop: 2854679000,
            },
            repeat: vec![],
            zone: None
        }];

        let expected_media_description = crate::MediaDescription {
            media: lines::Media {
                media: lines::media::MediaType::Audio,
                port: 49170,
                num_of_ports: None,
                proto: lines::media::ProtoType::RtpAvp,
                fmt: "0".into(),
            },
            info: None,
            connections: vec![lines::Connection {
                nettype: lines::common::Nettype::In,
                addrtype: lines::common::Addrtype::Ip4,
                connection_address: "10.47.16.6".parse::<IpAddr>().unwrap().into(),
            }],
            bandwidths: vec![lines::Bandwidth {
                bwtype: lines::bandwidth::Bwtype::As,
                bandwidth: 551,
            }],
            key: None,
            attributes: vec![],
        };

        assert_eq!(
            SessionDescription::try_from(tokenizer),
            Ok(SessionDescription {
                version: lines::Version::V0,
                origin: lines::Origin {
                    username: "Alice".into(),
                    sess_id: "2890844526".into(),
                    sess_version: "2890842807".into(),
                    nettype: lines::common::Nettype::In,
                    addrtype: lines::common::Addrtype::Ip4,
                    unicast_address: IpAddr::V4(Ipv4Addr::new(10, 47, 16, 5)),
                },
                session_name: lines::SessionName::new("-".into()),
                session_info: None,
                uri: None,
                emails: vec![],
                phones: vec![],
                connection: None,
                bandwidths: vec![],
                times: expected_times,
                key: None,
                attributes: vec![],
                media_descriptions: vec![expected_media_description]
            })
        );
    }

    #[test]
    fn from_tokenizer3() {
        let tokenizer = Tokenizer {
            version: "0".into(),
            origin: crate::tokenizers::origin::Tokenizer {
                username: "Alice",
                sess_id: "2890844526",
                sess_version: "2890842807",
                nettype: "IN",
                addrtype: "IP4",
                unicast_address: "10.47.16.5",
            },
            session_name: "-".into(),
            session_info: None,
            uri: None,
            emails: vec![],
            phones: vec![],
            connection: None,
            bandwidths: vec![],
            times: vec1![crate::tokenizers::time::Tokenizer {
                active: ("2854678930", "2854679000").into(),
                repeat: vec![],
                zone: None
            }],
            key: None,
            attributes: vec![],
            media_descriptions: vec![],
        };

        let expected_times = vec1![crate::Time {
            active: lines::Active {
                start: 2854678930,
                stop: 2854679000,
            },
            repeat: vec![],
            zone: None
        }];

        assert_eq!(
            SessionDescription::try_from(tokenizer),
            Ok(SessionDescription {
                version: lines::Version::V0,
                origin: lines::Origin {
                    username: "Alice".into(),
                    sess_id: "2890844526".into(),
                    sess_version: "2890842807".into(),
                    nettype: lines::common::Nettype::In,
                    addrtype: lines::common::Addrtype::Ip4,
                    unicast_address: IpAddr::V4(Ipv4Addr::new(10, 47, 16, 5)),
                },
                session_name: lines::SessionName::new("-".into()),
                session_info: None,
                uri: None,
                emails: vec![],
                phones: vec![],
                connection: None,
                bandwidths: vec![],
                times: expected_times,
                key: None,
                attributes: vec![],
                media_descriptions: vec![]
            })
        );
    }

    #[test]
    fn display1() {
        let sdp = concat!(
            "v=0\r\n",
            "o=Alice 2890844526 2890842807 IN IP4 10.47.16.5\r\n",
            "s=-\r\n",
            "i=A Seminar on the session description protocol\r\n",
            "u=http://www.example.com/seminars/sdp.pdf\r\n",
            "e=alice@example.com (Alice Smith)\r\n",
            "p=+1 911-345-1160\r\n",
            "c=IN IP4 10.47.16.5\r\n",
            "b=CT:1024\r\n",
            "t=2854678930 2854679000\r\n",
            "r=604800 3600 0 90000\r\n",
            "z=2882844526 -1h 2898848070 0h\r\n",
            "k=clear:password\r\n",
            "a=recvonly\r\n",
            "m=audio 49170 RTP/AVP 0\r\n",
            "i=audio media\r\n",
            "c=IN IP4 10.47.16.5\r\n",
            "c=IN IP4 10.47.16.6\r\n",
            "b=CT:1000\r\n",
            "b=AS:551\r\n",
            "k=prompt\r\n",
            "a=rtpmap:99 h232-199/90000\r\n",
            "a=rtpmap:90 h263-1998/90000\r\n"
        );

        let parsed_sdp = SessionDescription::try_from(Tokenizer::tokenize(sdp).unwrap().1).unwrap();
        assert_eq!(parsed_sdp.to_string(), sdp);
    }

    #[test]
    fn display2() {
        let sdp = concat!(
            "v=0\r\n",
            "o=Alice 2890844526 2890842807 IN IP4 10.47.16.5\r\n",
            "s=-\r\n",
            "t=2854678930 2854679000\r\n",
            "m=audio 49170 RTP/AVP 0\r\n",
            "c=IN IP4 10.47.16.6\r\n",
            "b=AS:551\r\n",
        );

        let parsed_sdp = SessionDescription::try_from(Tokenizer::tokenize(sdp).unwrap().1).unwrap();
        assert_eq!(parsed_sdp.to_string(), sdp);
    }

    #[test]
    fn display3() {
        let sdp = concat!(
            "v=0\r\n",
            "o=Alice 2890844526 2890842807 IN IP4 10.47.16.5\r\n",
            "s=-\r\n",
            "t=2854678930 2854679000\r\n",
        );

        let parsed_sdp = SessionDescription::try_from(Tokenizer::tokenize(sdp).unwrap().1).unwrap();
        assert_eq!(parsed_sdp.to_string(), sdp);
    }

    #[test]
    fn errors1() {
        let sdp = concat!(
            "v=0\r\n",
            "o=Alice 2890844526 2890842807 IN IP4 10.47.16.5\r\n",
            "s=-\r\n",
            "i=A Seminar on the session description protocol\r\n",
            "u=http://www.example.com/seminars/sdp.pdf\r\n",
            "e=alice@example.com (Alice Smith)\r\n",
            "p=+1 911-345-1160\r\n",
            "c=IN IP4 10.47.16\r\n",
            "b=CT:1024\r\n",
            "t=2854678930 2854679000\r\n",
            "r=604800 3600 0 90000\r\n",
            "z=2882844526 -1h 2898848070 0h\r\n",
            "k=clear:password\r\n",
            "a=recvonly\r\n",
            "m=audio 49170 RTP/AVP 0\r\n",
            "i=audio media\r\n",
            "c=IN IP4 10.47.16.5\r\n",
            "c=IN IP4 10.47.16.6\r\n",
            "b=CT:1000\r\n",
            "b=AS:551\r\n",
            "k=prompt\r\n",
            "a=rtpmap:99 h232-199/90000\r\n",
            "a=rtpmap:99 h263-1998/90000\r\n"
        );

        let parsed_sdp = SessionDescription::try_from(sdp);
        assert!(parsed_sdp.is_err());
    }
}
