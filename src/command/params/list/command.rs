use std::error;

use clap;

use super::executer::Executer;
use config;

pub struct Command<'c> {
    config: &'c config::command::Config,
}

impl<'c> Command<'c> {
    pub fn from_args(config: &'c config::command::Config, _args: &'c clap::ArgMatches<'c>) -> Self {
        trace!("command::params::list::Command::from_args");

        Command { config: config }
    }

    pub fn new(config: &'c config::command::Config) -> Self {
        trace!("command::params::list::Command::new");

        Command { config: config }
    }

    pub fn run(&self) -> Result<(), Box<dyn error::Error>> {
        trace!("command::params::list::Command::run");
        if let Some(params_config) = self.config.params.as_ref() {
            let exec = Executer::from_config(params_config);
            try!(exec.run());
        }
        Ok(())
    }
}
