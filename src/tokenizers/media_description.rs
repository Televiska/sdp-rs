use crate::tokenizers::{connection, key_optvalue, key_value, media, value};
use crate::TResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tokenizer<'a> {
    pub media: media::Tokenizer<'a>,
    pub info: Option<value::Tokenizer<'a, 'i'>>,
    pub connections: Vec<connection::Tokenizer<'a>>,
    pub bandwidths: Vec<key_value::Tokenizer<'a, 'b'>>,
    pub key: Option<key_optvalue::Tokenizer<'a, 'k'>>,
    pub attributes: Vec<key_optvalue::Tokenizer<'a, 'a'>>,
}

//TODO: add many0
impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a str) -> TResult<'a, Self> {
        use nom::multi::many1;

        let (rem, media) = media::Tokenizer::tokenize(part)?;
        let (rem, info) = match rem.starts_with("i=") {
            true => {
                let (rem, info) = value::Tokenizer::tokenize(rem)?;
                (rem, Some(info))
            }
            false => (rem, None),
        };
        let (rem, connections) = match rem.starts_with("c=") {
            true => many1(connection::Tokenizer::tokenize)(rem)?,
            false => (rem, vec![]),
        };
        let (rem, bandwidths) = match rem.starts_with("b=") {
            true => many1(key_value::Tokenizer::tokenize)(rem)?,
            false => (rem, vec![]),
        };
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

        Ok((
            rem,
            Tokenizer {
                media,
                info,
                connections,
                bandwidths,
                key,
                attributes,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer1() {
        let part = concat!(
            "m=audio 49170 RTP/AVP 0\r\n",
            "i=audio media\r\n",
            "c=IN IP4 10.47.16.5\r\n",
            "c=IN IP4 10.47.16.6\r\n",
            "b=CT:1000\r\n",
            "b=AS:551\r\n",
            "k=prompt\r\n",
            "a=rtpmap:99 h232-199/90000\r\n",
            "a=rtpmap:90 h263-1998/90000\r\nsomething"
        );

        assert_eq!(
            Tokenizer::tokenize(part),
            Ok((
                "something",
                Tokenizer {
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
                }
            ))
        );
    }

    #[test]
    fn tokenizer2() {
        let part = concat!("m=audio 49170 RTP/AVP 0\r\nsomething",);

        assert_eq!(
            Tokenizer::tokenize(part),
            Ok((
                "something",
                Tokenizer {
                    media: media::Tokenizer {
                        media: "audio",
                        port: "49170".into(),
                        proto: "RTP/AVP",
                        fmt: "0"
                    },
                    info: None,
                    connections: vec![],
                    bandwidths: vec![],
                    key: None,
                    attributes: vec![],
                }
            ),)
        );
    }
}
