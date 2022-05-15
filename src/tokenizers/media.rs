use crate::TResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tokenizer<'a> {
    pub media: &'a str,
    pub port: PortTokenizer<'a>,
    pub proto: &'a str,
    pub fmt: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn tokenize(part: &'a str) -> TResult<'a, Self> {
        use crate::parser_utils::*;
        use nom::{bytes::complete::tag, sequence::preceded};

        let (rem, media) = preceded(tag("m="), until_space)(part)?;
        let (rem, port_with_num_of_ports) = until_space(rem)?;
        let (rem, proto) = until_space(rem)?;
        let (rem, fmt) = until_newline(rem)?;
        let (_, port) = PortTokenizer::tokenize(port_with_num_of_ports)?;

        Ok((
            rem,
            Tokenizer {
                media,
                port,
                proto,
                fmt,
            },
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PortTokenizer<'a> {
    pub port: &'a str,
    pub num_of_ports: Option<&'a str>,
}

impl<'a> PortTokenizer<'a> {
    pub fn tokenize(part: &'a str) -> TResult<'a, Self> {
        use nom::{
            branch::alt,
            bytes::complete::{tag, take_until},
            combinator::rest,
            sequence::terminated,
        };

        let (num_of_ports, port) = alt((terminated(take_until("/"), tag("/")), rest))(part)?;
        let num_of_ports = match num_of_ports.is_empty() {
            true => None,
            false => Some(num_of_ports),
        };

        Ok(("", (port, num_of_ports).into()))
    }
}

impl<'a> From<(&'a str, Option<&'a str>)> for PortTokenizer<'a> {
    fn from((port, num_of_ports): (&'a str, Option<&'a str>)) -> Self {
        Self { port, num_of_ports }
    }
}

impl<'a> From<(&'a str, &'a str)> for PortTokenizer<'a> {
    fn from((port, num_of_ports): (&'a str, &'a str)) -> Self {
        Self {
            port,
            num_of_ports: Some(num_of_ports),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer1() {
        let media = concat!("m=audio 49170 RTP/AVP 0\r\nsomething");

        assert_eq!(
            Tokenizer::tokenize(media),
            Ok((
                "something",
                Tokenizer {
                    media: "audio",
                    port: ("49170", None).into(),
                    proto: "RTP/AVP",
                    fmt: "0",
                }
            )),
        );
    }

    #[test]
    fn tokenizer2() {
        let media = concat!("m=video 49170/2 RTP/AVP 31\r\nsomething");

        assert_eq!(
            Tokenizer::tokenize(media),
            Ok((
                "something",
                Tokenizer {
                    media: "video",
                    port: ("49170", "2").into(),
                    proto: "RTP/AVP",
                    fmt: "31",
                }
            )),
        );
    }
}
