use std::error;

use clap;

use super::executer::Executer;
use config;

pub struct Command<'c> {
    config: &'c config::command::Config,
    name: &'c str,
    value: &'c str,
}

impl<'c> Command<'c> {
    pub fn from_args(config: &'c config::command::Config, args: &'c clap::ArgMatches<'c>) -> Self {
        trace!("command::params::put::Command::from_args");
        Command {
            config: config,
            name: args.value_of("NAME").unwrap(),
            value: args.value_of("VALUE").unwrap(),
        }
    }

    pub fn new(config: &'c config::command::Config, name: &'c str, value: &'c str) -> Self {
        trace!("command::params::put::Command::new");
        Command {
            config: config,
            name: name,
            value: value,
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn error::Error>> {
        trace!("command::params::put::Command::run");
        if let Some(params_config) = self.config.params.as_ref() {
            let exec = Executer::from_config(&params_config);

            try!(exec.run(self.name, self.value));
        }
        Ok(())
    }
}
