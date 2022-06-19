use std::{error::Error as StdError, fmt};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TokenizerError {
    pub context: String,
    pub nom_kind: Option<nom::error::ErrorKind>,
}

impl TokenizerError {
    pub fn part_with_error<I>(
        element: &'static str,
        input: I,
        error: nom::Err<TokenizerError>,
    ) -> Self
    where
        I: std::fmt::Display,
    {
        Self {
            context: format!(
                "failed to tokenize {} ( {} ): {}",
                element,
                nom_inner(error),
                input
            ),
            nom_kind: None,
        }
    }

    pub fn part_with_input<I>(element: &'static str, input: I) -> Self
    where
        I: std::fmt::Display,
    {
        Self {
            context: format!("failed to tokenize {}: {}", element, input),
            nom_kind: None,
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<nom::Err<Self>> for TokenizerError {
    fn into(self) -> nom::Err<Self> {
        nom::Err::Error(self)
    }
}

/*
#[allow(clippy::from_over_into)]
impl<'a, T> Into<crate::IResult<'a, T>> for TokenizerError {
    fn into(self) -> crate::IResult<'a, T> {
        Err(nom::Err::Error(self))
    }
}*/

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tokenizer error: {}", self.context)
    }
}

impl StdError for TokenizerError {}

impl<T: std::fmt::Display> nom::error::ParseError<T> for TokenizerError {
    fn from_error_kind(input: T, kind: nom::error::ErrorKind) -> Self {
        Self {
            context: format!(
                "could not tokenize or was expecting something else before: {}",
                input
            ),
            nom_kind: Some(kind),
        }
    }
    fn append(input: T, kind: nom::error::ErrorKind, _: Self) -> Self {
        Self {
            context: format!("could not tokenize: {}", input),
            nom_kind: Some(kind),
        }
    }
}

fn nom_inner(e: nom::Err<TokenizerError>) -> String {
    match e {
        nom::Err::Incomplete(_) => "parsing requires more data".into(),
        nom::Err::Failure(c) => c.context,
        nom::Err::Error(c) => c.context,
    }
}
