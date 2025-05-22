// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketTitle` type,
//   enforcing that the title is not empty and is not longer than 50 characters.
//   Implement the traits required to make the tests pass too.

use core::fmt;
use std::{clone, error::Error, str::FromStr};
#[derive(Debug, PartialEq, Clone)]
pub struct TicketTitle(String);
#[derive(Debug)]
pub struct TicketTitleError(&'static str);

impl Error for TicketTitleError {}

impl fmt::Display for TicketTitleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for TicketTitle {
    type Err = TicketTitleError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(TicketTitleError("The title cannot be empty".into()))
        } else if s.len() > 50 {
            Err(TicketTitleError("The title cannot be longer than 50 bytes").into())
        } else {
            Ok(TicketTitle(s.to_string()))
        }
    }
}

impl TryFrom<String> for TicketTitle {
    type Error = TicketTitleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        TicketTitle::from_str(&value)
    }
}

impl TryFrom<&str> for TicketTitle {
    type Error = TicketTitleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        TicketTitle::from_str(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let title = TicketTitle::try_from("A title".to_string()).unwrap();
        assert_eq!(title.0, "A title");
    }

    #[test]
    fn test_try_from_empty_string() {
        let err = TicketTitle::try_from("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be empty");
    }

    #[test]
    fn test_try_from_long_string() {
        let title =
            "A title that's definitely longer than what should be allowed in a development ticket"
                .to_string();
        let err = TicketTitle::try_from(title).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be longer than 50 bytes");
    }

    #[test]
    fn test_try_from_str() {
        let title = TicketTitle::try_from("A title").unwrap();
        assert_eq!(title.0, "A title");
    }
}
