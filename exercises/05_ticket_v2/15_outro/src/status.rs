// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `Status` enum.
//  The parsing should be case-insensitive.

use std::str::FromStr;
#[derive(PartialEq, Debug, Clone)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}
#[derive(Debug)]
pub struct ParseStatusError {
    invalid_status: String,
}

impl FromStr for Status {
    type Err = ParseStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.eq_ignore_ascii_case("todo") {
            Ok(Status::ToDo)
        } else if s.eq_ignore_ascii_case("inprogress") {
            Ok(Status::InProgress)
        } else if s.eq_ignore_ascii_case("done") {
            Ok(Status::Done)
        } else {
            Err(ParseStatusError { invalid_status: s.into() })
        }
    }
}

impl TryFrom<String> for Status {
    type Error = ParseStatusError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Status::from_str(&value)
    }
}

impl TryFrom<&str> for Status {
    type Error = ParseStatusError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Status::from_str(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("ToDO").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done").unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_invalid() {
        let status = Status::try_from("Invalid");
        assert!(status.is_err());
    }
}
