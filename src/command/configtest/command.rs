use std::error;

use clap;
use serde_yaml;

use crate::config;

pub struct Command<'c> {
    config: &'c config::command::Config,
}

impl<'c> Command<'c> {
    pub fn from_args(config: &'c config::command::Config, _args: &'c clap::ArgMatches<'c>) -> Self {
        trace!("command::config::Command::from_args");

        Command { config: config }
    }

    pub fn new(config: &'c config::command::Config) -> Self {
        trace!("command::config::Command::new");

        Command { config: config }
    }

    pub fn run(&self) -> Result<(), Box<dyn error::Error>> {
        trace!("command::config::Command::run");

        println!("{}", serde_yaml::to_string(&self.config).unwrap());

        Ok(())
    }
}
