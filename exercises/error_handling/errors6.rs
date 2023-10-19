#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
enum ParsePosNonzeroError {
    ParseInt(std::num::ParseIntError),
    Creation(CreationError),
}

use std::fmt;

impl fmt::Display for ParsePosNonzeroError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ParseInt(err) => write!(f, "ParseIntError: {}", err),
            Self::Creation(err) => write!(f, "CreationError: {:?}", err),
        }
    }
}

use std::error::Error;

impl Error for ParsePosNonzeroError {}

fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
    let value: i64 = s.parse().map_err(ParsePosNonzeroError::ParseInt)?;
    PositiveNonzeroInteger::new(value).map_err(ParsePosNonzeroError::Creation)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        assert!(matches!(
            parse_pos_nonzero("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            parse_pos_nonzero("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            parse_pos_nonzero("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42).unwrap();
        assert_eq!(parse_pos_nonzero("42"), Ok(x));
    }
}
