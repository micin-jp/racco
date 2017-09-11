extern crate env_logger;
extern crate racco;

use racco::command::MainCommand;

fn main() {
    env_logger::init().unwrap();

    let _ = MainCommand::run();
}
