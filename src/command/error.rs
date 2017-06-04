use std::error;
use std::fmt;

#[derive(Debug)]
pub enum CommandError {
  Unknown,
}

impl fmt::Display for CommandError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      CommandError::Unknown => write!(f, "Unexpected error occurred"),
    }
  }
}

impl error::Error for CommandError {
  fn description(&self) -> &str {
      match *self {
        CommandError::Unknown => "Unexpected error occurred",
      }
  }

  fn cause(&self) -> Option<&error::Error> {
      match *self {
        CommandError::Unknown => None,
      }
  }
}
