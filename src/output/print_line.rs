use termion::{color};

pub struct PrintLine {
}

impl PrintLine {

  pub fn print(msg: &str) {
    println!("{}", msg)
  }

  pub fn info(msg: &str) {
    println!("{}", msg)
  }

  pub fn warn(msg: &str) {
    eprintln!("{}{}{}", color::Fg(color::Yellow), msg, color::Fg(color::Reset))
  }

  pub fn error(msg: &str) {
    eprintln!("{}{}{}", color::Fg(color::Red), msg, color::Fg(color::Reset))
  }

  pub fn success(msg: &str) {
    println!("{}{}{}", color::Fg(color::Green), msg, color::Fg(color::Reset))
  }
}