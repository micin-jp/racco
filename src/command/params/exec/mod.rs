mod executer;
mod command;

pub use self::executer::Executer;
pub use self::command::Command;

type Program<'a> = &'a str;
type Arguments<'a> = Vec<&'a str>;