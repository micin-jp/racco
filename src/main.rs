#[macro_use]
extern crate log;

extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate serde_yaml;

extern crate handlebars;

extern crate clap;

extern crate hyper;

extern crate rusoto_core;
extern crate rusoto_credential;
extern crate rusoto_ecs;
extern crate rusoto_events;
extern crate rusoto_ssm;

extern crate tabwriter;
extern crate termion;

extern crate semver;

pub mod command;
pub mod config;
pub mod output;

use crate::command::MainCommand;

#[tokio::main]
async fn main() {
    env_logger::init().unwrap();

    ::std::process::exit(match MainCommand::run().await {
        Ok(_res) => 0,
        Err(_err) => 1,
    });
}
