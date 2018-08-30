mod command;
mod executer;

pub use self::command::Command;
pub use self::executer::Executer;

type Program<'a> = &'a str;
type Arguments<'a> = Vec<&'a str>;
