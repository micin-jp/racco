extern crate env_logger;
extern crate racco;

use racco::command::MainCommand;

fn main() {
    env_logger::init().unwrap();

    ::std::process::exit(match MainCommand::run() {
        Ok(_res) => 0,
        Err(_err) => 1,
    });
}
