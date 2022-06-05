use crate::{tokenizers::*, TResult, TokenizerError};
use std::convert::TryInto;
use vec1::Vec1;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tokenizer<'a> {
    pub version: value::Tokenizer<'a, 'v'>,
    pub origin: origin::Tokenizer<'a>,
    pub session_name: value::Tokenizer<'a, 's'>,
    pub session_info: Option<value::Tokenizer<'a, 'i'>>,
    pub uri: Option<value::Tokenizer<'a, 'u'>>,
    pub emails: Vec<value::Tokenizer<'a, 'e'>>,
    pub phones: Vec<value::Tokenizer<'a, 'p'>>,
    pub connection: Option<connection::Tokenizer<'a>>,
    pub bandwidths: Vec<key_value::Tokenizer<'a, 'b'>>,
    pub times: Vec1<time::Tokenizer<'a>>,
    pub key: Option<key_optvalue::Tokenizer<'a, 'k'>>,
    pub attributes: Vec<key_optvalue::Tokenizer<'a, 'a'>>,
    pub media_descriptions: Vec<media_description::Tokenizer<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a str) -> TResult<'a, Self> {
        use nom::multi::many1;

        let (rem, version) = value::Tokenizer::tokenize(part)?;
        let (rem, origin) = origin::Tokenizer::tokenize(rem)?;
        let (rem, session_name) = value::Tokenizer::tokenize(rem)?;
        let (rem, session_info) = match rem.starts_with("i=") {
            true => {
                let (rem, info) = value::Tokenizer::tokenize(rem)?;
                (rem, Some(info))
            }
            false => (rem, None),
        };
        let (rem, uri) = match rem.starts_with("u=") {
            true => {
                let (rem, uri) = value::Tokenizer::tokenize(rem)?;
                (rem, Some(uri))
            }
            false => (rem, None),
        };
        let (rem, emails) = match rem.starts_with("e=") {
            true => many1(value::Tokenizer::tokenize)(rem)?,
            false => (rem, vec![]),
        };
        let (rem, phones) = match rem.starts_with("p=") {
            true => many1(value::Tokenizer::tokenize)(rem)?,
            false => (rem, vec![]),
        };
        let (rem, connection) = match rem.starts_with("c=") {
            true => {
                let (rem, phone) = connection::Tokenizer::tokenize(rem)?;
                (rem, Some(phone))
            }
            false => (rem, None),
        };
        let (rem, bandwidths) = match rem.starts_with("b=") {
            true => many1(key_value::Tokenizer::tokenize)(rem)?,
            false => (rem, vec![]),
        };
        let (rem, times) = many1(time::Tokenizer::tokenize)(rem)?;
        let (rem, key) = match rem.starts_with("k=") {
            true => {
                let (rem, key) = key_optvalue::Tokenizer::tokenize(rem)?;
                (rem, Some(key))
            }
            false => (rem, None),
        };
        let (rem, attributes) = match rem.starts_with("a=") {
            true => many1(key_optvalue::Tokenizer::tokenize)(rem)?,
            false => (rem, vec![]),
        };
        let (rem, media_descriptions) = match rem.starts_with("m=") {
            true => many1(media_description::Tokenizer::tokenize)(rem)?,
            false => (rem, vec![]),
        };

        Ok((
            rem,
            Tokenizer {
                version,
                origin,
                session_name,
                session_info,
                uri,
                emails,
                phones,
                connection,
                bandwidths,
                times: times.try_into().map_err(|_| {
                    nom::Err::Error(TokenizerError::from(("time", "missing line(s)")))
                })?,
                key,
                attributes,
                media_descriptions,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vec1::vec1;

    #[test]
    fn tokenizer1() {
        let part = concat!(
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

        assert_eq!(
            Tokenizer::tokenize(part),
            Ok((
                "",
                Tokenizer {
                    version: "0".into(),
                    origin: origin::Tokenizer {
                        username: "Alice",
                        sess_id: "2890844526",
                        sess_version: "2890842807",
                        nettype: "IN",
                        addrtype: "IP4",
                        unicast_address: "10.47.16.5"
                    },
                    session_name: "-".into(),
                    session_info: Some("A Seminar on the session description protocol".into()),
                    uri: Some("http://www.example.com/seminars/sdp.pdf".into()),
                    emails: vec!["alice@example.com (Alice Smith)".into()],
                    phones: vec!["+1 911-345-1160".into()],
                    connection: Some(connection::Tokenizer {
                        nettype: "IN",
                        addrtype: "IP4",
                        connection_address: ("10.47.16.5").into()
                    }),
                    bandwidths: vec![("CT", "1024").into()],
                    times: vec1![time::Tokenizer {
                        active: ("2854678930", "2854679000").into(),
                        repeat: vec![("604800", "3600", vec!["0", "90000"]).into(),],
                        zone: Some(vec![("2882844526", "-1h"), ("2898848070", "0h")].into())
                    }],
                    key: Some(("clear", "password").into()),
                    attributes: vec!["recvonly".into()],
                    media_descriptions: vec![media_description::Tokenizer {
                        media: media::Tokenizer {
                            media: "audio",
                            port: "49170".into(),
                            proto: "RTP/AVP",
                            fmt: "0"
                        },
                        info: Some("audio media".into()),
                        connections: vec![
                            connection::Tokenizer {
                                nettype: "IN",
                                addrtype: "IP4",
                                connection_address: "10.47.16.5".into(),
                            },
                            connection::Tokenizer {
                                nettype: "IN",
                                addrtype: "IP4",
                                connection_address: "10.47.16.6".into(),
                            }
                        ],
                        bandwidths: vec![("CT", "1000").into(), ("AS", "551").into()],
                        key: Some("prompt".into()),
                        attributes: vec![
                            ("rtpmap", "99 h232-199/90000").into(),
                            ("rtpmap", "90 h263-1998/90000").into()
                        ],
                    }]
                }
            ))
        );
    }

    #[test]
    fn tokenizer2() {
        let part = concat!(
            "v=0\r\n",
            "o=Alice 2890844526 2890842807 IN IP4 10.47.16.5\r\n",
            "s=-\r\n",
            "t=2854678930 2854679000\r\n",
            "m=audio 49170 RTP/AVP 0\r\n",
            "c=IN IP4 10.47.16.6\r\n",
            "b=AS:551\r\n",
        );

        assert_eq!(
            Tokenizer::tokenize(part),
            Ok((
                "",
                Tokenizer {
                    version: "0".into(),
                    origin: origin::Tokenizer {
                        username: "Alice",
                        sess_id: "2890844526",
                        sess_version: "2890842807",
                        nettype: "IN",
                        addrtype: "IP4",
                        unicast_address: "10.47.16.5"
                    },
                    session_name: "-".into(),
                    session_info: None,
                    uri: None,
                    emails: vec![],
                    phones: vec![],
                    connection: None,
                    bandwidths: vec![],
                    times: vec1![time::Tokenizer {
                        active: ("2854678930", "2854679000").into(),
                        repeat: vec![],
                        zone: None
                    }],
                    key: None,
                    attributes: vec![],
                    media_descriptions: vec![media_description::Tokenizer {
                        media: media::Tokenizer {
                            media: "audio",
                            port: "49170".into(),
                            proto: "RTP/AVP",
                            fmt: "0"
                        },
                        info: None,
                        connections: vec![connection::Tokenizer {
                            nettype: "IN",
                            addrtype: "IP4",
                            connection_address: "10.47.16.6".into(),
                        }],
                        bandwidths: vec![("AS", "551").into()],
                        key: None,
                        attributes: vec![],
                    }]
                }
            ))
        );
    }

    #[test]
    fn tokenizer3() {
        let part = concat!(
            "v=0\r\n",
            "o=Alice 2890844526 2890842807 IN IP4 10.47.16.5\r\n",
            "s=-\r\n",
            "t=2854678930 2854679000\r\n",
        );

        assert_eq!(
            Tokenizer::tokenize(part),
            Ok((
                "",
                Tokenizer {
                    version: "0".into(),
                    origin: origin::Tokenizer {
                        username: "Alice",
                        sess_id: "2890844526",
                        sess_version: "2890842807",
                        nettype: "IN",
                        addrtype: "IP4",
                        unicast_address: "10.47.16.5"
                    },
                    session_name: "-".into(),
                    session_info: None,
                    uri: None,
                    emails: vec![],
                    phones: vec![],
                    connection: None,
                    bandwidths: vec![],
                    times: vec1![time::Tokenizer {
                        active: ("2854678930", "2854679000").into(),
                        repeat: vec![],
                        zone: None
                    }],
                    key: None,
                    attributes: vec![],
                    media_descriptions: vec![]
                }
            ))
        );
    }
}
