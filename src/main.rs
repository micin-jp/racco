#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;

extern crate clap;

extern crate hyper;

extern crate rusoto_core;
extern crate rusoto_credential;
extern crate rusoto_ecs;
extern crate rusoto_ssm;

mod command;
mod config;

use command::{MainCommand};

fn main() {
    env_logger::init().unwrap();

    MainCommand::run();
}