use std::error;
use std::fmt;

#[derive(Debug)]
pub enum CommandError {
    CommandNotFound,
    Unknown,
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CommandError::CommandNotFound => write!(f, "Unknown command"),
            CommandError::Unknown => write!(f, "Unexpected error occurred"),
        }
    }
}

impl error::Error for CommandError {
    fn description(&self) -> &str {
        match *self {
            CommandError::CommandNotFound => "Unknown command",
            CommandError::Unknown => "Unexpected error occurred",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CommandError::CommandNotFound => None,
            CommandError::Unknown => None,
        }
    }
}
