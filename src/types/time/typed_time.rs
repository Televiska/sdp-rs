use crate::Error;
use chrono::Duration;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Copy)]
pub enum TypedTime {
    Seconds(Duration),
    Minutes(Duration),
    Hours(Duration),
    Days(Duration),
    None(Duration),
}

impl From<Duration> for TypedTime {
    fn from(duration: Duration) -> Self {
        Self::None(duration)
    }
}

impl From<TypedTime> for Duration {
    fn from(from: TypedTime) -> Self {
        match from {
            TypedTime::Seconds(duration) => duration,
            TypedTime::Minutes(duration) => duration,
            TypedTime::Hours(duration) => duration,
            TypedTime::Days(duration) => duration,
            TypedTime::None(duration) => duration,
        }
    }
}

impl<'a> TryFrom<&'a str> for TypedTime {
    type Error = Error;

    fn try_from(part: &'a str) -> Result<Self, Self::Error> {
        let typed_time = match part.chars().last() {
            Some('s') => Self::Seconds(Duration::seconds(part[0..part.len() - 1].parse::<i64>()?)),
            Some('m') => Self::Minutes(Duration::minutes(part[0..part.len() - 1].parse::<i64>()?)),
            Some('h') => Self::Hours(Duration::hours(part[0..part.len() - 1].parse::<i64>()?)),
            Some('d') => Self::Days(Duration::days(part[0..part.len() - 1].parse::<i64>()?)),
            _ => Self::None(Duration::seconds(part.parse::<i64>()?)),
        };

        Ok(typed_time)
    }
}

impl std::fmt::Display for TypedTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Seconds(duration) => write!(f, "{}s", duration.num_seconds()),
            Self::Minutes(duration) => write!(f, "{}m", duration.num_minutes()),
            Self::Hours(duration) => write!(f, "{}h", duration.num_hours()),
            Self::Days(duration) => write!(f, "{}d", duration.num_days()),
            Self::None(duration) => write!(f, "{}", duration.num_seconds()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_correctly1() {
        assert_eq!(
            TypedTime::try_from("3000"),
            Ok(TypedTime::None(Duration::seconds(3000)))
        );
    }

    #[test]
    fn parses_correctly2() {
        assert_eq!(
            TypedTime::try_from("300s"),
            Ok(TypedTime::Seconds(Duration::seconds(300)))
        );
    }

    #[test]
    fn parses_correctly3() {
        assert_eq!(
            TypedTime::try_from("-17m"),
            Ok(TypedTime::Minutes(Duration::minutes(-17)))
        );
    }

    #[test]
    fn parses_correctly4() {
        assert_eq!(
            TypedTime::try_from("25h"),
            Ok(TypedTime::Hours(Duration::hours(25)))
        );
    }

    #[test]
    fn parses_correctly5() {
        assert_eq!(
            TypedTime::try_from("7d"),
            Ok(TypedTime::Days(Duration::days(7)))
        );
    }

    #[test]
    fn parses_correctly6() {
        assert_eq!(
            TypedTime::try_from("0"),
            Ok(TypedTime::None(Duration::seconds(0)))
        );
    }

    #[test]
    fn parses_correctly7() {
        assert_eq!(
            TypedTime::try_from("-0"),
            Ok(TypedTime::None(Duration::seconds(0)))
        );
    }

    #[test]
    fn parses_correctly8() {
        assert_eq!(
            TypedTime::try_from("-0m"),
            Ok(TypedTime::Minutes(Duration::seconds(0)))
        );
    }

    #[test]
    fn display1() {
        let typed_time = TypedTime::Hours(Duration::hours(3));

        assert_eq!(typed_time.to_string(), "3h");
    }

    #[test]
    fn display2() {
        let typed_time = TypedTime::Minutes(Duration::minutes(-3));

        assert_eq!(typed_time.to_string(), "-3m");
    }

    #[test]
    fn display3() {
        let typed_time = TypedTime::Minutes(Duration::minutes(3));

        assert_eq!(typed_time.to_string(), "3m");
    }
}
